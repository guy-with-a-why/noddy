# Bug: PageAlreadyMapped panic at boot

## Symptom

Running the OS image in QEMU produces the following panic before the kernel starts:

```
panicked at src\page_table.rs:105:25:
* failed to map segment starting at Page[4kiB](0x1000): failed to map page
  Page[4KiB](0x1000) to frame PhysFrame[4KiB](0x400000): PageAlreadyMapped
```

The panic originates inside `bootloader 0.9.x` (`src/page_table.rs:105`), not in the kernel itself.

---

## Root Cause

### The kernel ELF is compiled as a PIE

Inspecting the kernel binary with `llvm-objdump -p`:

```
LOAD off 0x000 vaddr 0x00000000 align 2**12  filesz 0xa18   flags r--
LOAD off 0xa20 vaddr 0x00001a20 align 2**12  filesz 0x124c  flags r-x
LOAD off 0x1c70 vaddr 0x00003c70 align 2**12 filesz 0x338   flags rw-

DYNAMIC ...
  FLAGS_1   0x0000000008000001   ← bit 27 = DF_1_PIE
```

The `DF_1_PIE` flag confirms the binary is a **Position-Independent Executable**. The
`x86_64-unknown-none` Rust target in recent nightly toolchains produces PIE by default. PIE
executables use virtual addresses starting at `0x0`; a runtime loader is expected to
relocate them. The bootloader 0.9.x does **not** perform PIE relocation — it maps ELF
segments literally at the virtual addresses stated in the file.

### Why LOAD 1 succeeds and LOAD 2 fails

`bootloader/src/page_table.rs::map_segment()` translates each physical frame of a LOAD
segment to a virtual page and calls `map_to`:

```
LOAD 1  virt_start=0x0000  file_offset=0x000  phys=0x400000+0x000=0x400000
        frames [0x400000..=0x400000]
        → maps page 0x0 → frame 0x400000          ✓  succeeds

LOAD 2  virt_start=0x1a20  file_offset=0xa20  phys=0x400000+0xa20=0x400a20
        start_frame = containing(0x400a20) = 0x400000
        end_frame   = containing(0x401c6b) = 0x401000
        → maps page 0x1000 → frame 0x400000       ✗  PageAlreadyMapped
        → maps page 0x2000 → frame 0x401000       (never reached)
```

Virtual page `0x0` (the null page) is deliberately **not** mapped by the bootloader's
stage_3 assembly — a standard null-pointer guard. Virtual page `0x1000` **is** already
mapped because the bootloader's own page-table structures reside at **physical** `0x1000`
and stage_3 identity-maps them (virtual `0x1000` → physical `0x1000`) so they remain
accessible after the switch to 64-bit long mode.

`bootloader_main` (lines 234-240) only unmaps the kernel binary's identity mapping at
virtual `0x400000` before calling `map_kernel`. It never unmaps the page-table region at
`0x1000`. When LOAD 2 tries to install virtual page `0x1000` → physical frame `0x400000`,
the entry already exists and `map_to` returns `PageAlreadyMapped`.

### Summary table

| Virtual page | Stage-3 identity map? | Outcome |
|---|---|---|
| `0x0` | No (null-page guard) | LOAD 1 succeeds |
| `0x1000` | Yes → physical `0x1000` | LOAD 2 panics |

---

## Fix

See the changes applied in the same commit:

1. **`.cargo/config.toml`** — added `rustflags = ["-C", "relocation-model=static"]` under
   `[target.x86_64-unknown-none]` to disable PIE and produce a static `ET_EXEC` ELF.

2. **`linker.ld`** — linker script placing the kernel at virtual `0x400000`.  
   This is the address where the bootloader physically loads the kernel binary; the bootloader
   explicitly unmaps the identity mapping for this range before calling `map_kernel`, so the
   virtual pages are free when segment mapping runs.

3. **`build.rs`** — passes `-Tlinker.ld` to LLD via `cargo:rustc-link-arg`.

---

## Background

This project follows Phil Oppermann's _Writing an OS in Rust_ course
(https://os.phil-opp.com/) which uses `bootloader 0.9.x`. Phil's original tutorial used a
custom `x86_64-blog_os.json` target spec with `"relocation-model": "static"` baked in. When
using the standard `x86_64-unknown-none` target on recent nightly toolchains, the
`relocation-model=static` flag must be supplied explicitly.
