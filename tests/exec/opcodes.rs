// SPDX-License-Identifier: LGPL-2.1
use jzvm::exec;
use jzvm::exec::{CodeBuf, Context, ExitCondition, State};
use jzvm::op;

struct Test<'a> {
    code: &'a [u8],
    before: TestState<'a>,
    after: TestState<'a>,
    exit: ExitCondition,
}

impl<'a> Test<'a> {
    fn new(code: &'a [u8]) -> Self {
        Test {
            code,
            before: TestState::new(),
            after: TestState::new(),
            exit: ExitCondition::OpcodeHandler(op::breakpoint),
        }
    }

    fn before(self, before: TestState<'a>) -> Self {
        Self { before, ..self }
    }

    fn after(self, after: TestState<'a>) -> Self {
        Self { after, ..self }
    }

    fn exit(self, exit: ExitCondition) -> Self {
        Self { exit, ..self }
    }

    unsafe fn run(&self) {
        let mut exec = exec::new().unwrap();
        let code: CodeBuf = self.code.try_into().unwrap();
        let mut stack: Vec<usize> = self.before.stack.into();
        let mut locals: Vec<usize> = self.before.locals.into();

        // Make some space for additional stack elements
        for _ in 0..10 {
            stack.push(0)
        }

        let mut ctx = Context {
            code: code.as_ref(),
            stack: &mut stack,
            locals: &mut locals,
        };

        let mut state = State {
            pc: self.before.pc,
            sp: self.before.stack.len(),
        };

        let exit = exec.execute(&mut ctx, &mut state);

        assert_eq!(exit, Err(self.exit));
        assert_eq!(&ctx.stack[..state.sp], self.after.stack);
        assert_eq!(&ctx.locals[..], self.after.locals);
    }
}

#[derive(Default)]
struct TestState<'a> {
    pc: usize,
    stack: &'a [usize],
    locals: &'a [usize],
}

impl<'a> TestState<'a> {
    fn new() -> Self {
        Self::default()
    }

    fn pc(self, pc: usize) -> Self {
        TestState { pc, ..self }
    }

    fn stack(self, stack: &'a [usize]) -> Self {
        TestState { stack, ..self }
    }

    fn locals(self, locals: &'a [usize]) -> Self {
        TestState { locals, ..self }
    }
}

#[test]
#[should_panic]
fn test_stack_is_checked() {
    unsafe {
        // Here we modify the stack, and performatively don't expect it
        Test::new(&[op::aconst_null, op::breakpoint])
            .after(TestState::new().pc(1))
            .run()
    }
}

#[test]
#[should_panic]
fn test_locals_are_checked() {
    unsafe {
        // Store int 1 into local, and performatively don't expect it
        Test::new(&[op::iconst_1, op::istore_0, op::breakpoint])
            .before(TestState::new().locals(&[0]))
            .after(TestState::new().pc(2).locals(&[0]))
            .run()
    }
}

#[test]
fn test_00_nop() {
    assert_eq!(0x00, op::nop);
    unsafe {
        Test::new(&[op::nop, op::breakpoint])
            .after(TestState::new().pc(1))
            .run()
    }
}

#[test]
fn test_01_aconst_null() {
    assert_eq!(0x01, op::aconst_null);
    unsafe {
        Test::new(&[op::aconst_null, op::breakpoint])
            .after(TestState::new().pc(1).stack(&[0]))
            .run()
    }
}

#[test]
fn test_02_iconst_m1() {
    assert_eq!(0x02, op::iconst_m1);
    unsafe {
        Test::new(&[op::iconst_m1, op::breakpoint])
            .after(TestState::new().pc(1).stack(&[0xffffffff]))
            .run()
    }
}

#[test]
fn test_3b_istore_0() {
    assert_eq!(0x3b, op::istore_0);
    unsafe {
        Test::new(&[op::istore_0, op::breakpoint])
            .before(TestState::new().locals(&[0]).stack(&[42]))
            .after(TestState::new().pc(1).locals(&[42]))
            .run()
    }
}

#[test]
fn test_cb_unknown_opcodes() {
    for op in 0xcb..=0xfe {
        unsafe {
            Test::new(&[op, op::breakpoint])
                .exit(ExitCondition::OpcodeHandler(op))
                .run()
        }
    }
}

#[test]
#[ignore]
fn test_ff_impdep2_sigtrap() {
    // TODO: check that opcode 0xff (impdep2) raises a SIGTRAP
}
