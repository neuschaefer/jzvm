// SPDX-License-Identifier: LGPL-2.1
//use jzvm::exec;

#[test]
#[ignore]
fn demand_paging() {
    // allocate a page but don't populate it (mmap(anon))
    // use this page as stack
    // execute jz code that pushs some values
    // verify that the code terminates
}
