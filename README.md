# win_subst
**A tool library, the functions refer to the windows command subst, associates a path with a drive letter.**

# Usage

**dependency**:
```bash
cargo add win_subst
```

**import**
```rust
use win_subst::add;
```

**call**
```rust
add("<drive1>:", "[<drive2>:]<path>");
```

* `<drive1>:` - **Specifies the virtual drive to which you want to assign a path.**
* `[<drive2>:]<path>` - **Specifies the physical drive and path that you want to assign to a virtual drive.**
