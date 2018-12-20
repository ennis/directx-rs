pub unsafe fn wstrlen(mut pwstr: *const u16) -> usize {
    let mut len = 0;
    while *pwstr != 0 {
        len += 1;
        pwstr = pwstr.offset(1);
    }
    len
}
