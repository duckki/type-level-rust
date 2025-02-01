use crate::nat::*;

type One = Succ<Zero>;
type Two = Succ<One>;

mod thread {
    use super::*;

    pub struct Context<N: Nat> {
        _current_lock_level: std::marker::PhantomData<N>,
    }

    pub trait Thread {
        fn run<'a>(&'a self, ctx: &'a mut Context<Zero>);
    }

    pub fn spawn(main_thread: impl Thread) {
        let mut ctx = Context::<Zero> {
            _current_lock_level: std::marker::PhantomData,
        };
        main_thread.run(&mut ctx);
    }

    pub struct Mutex<N: Nat> {
        // No public constructors
        _lock_level: std::marker::PhantomData<N>,
    }

    pub fn mutex_one() -> Mutex<One> {
        Mutex {
            _lock_level: std::marker::PhantomData,
        }
    }

    pub fn mutex_two() -> Mutex<Two> {
        Mutex {
            _lock_level: std::marker::PhantomData,
        }
    }

    // Can only lock a mutex with a number that is higher than the context.
    pub fn lock_and_run<N: Nat, M: Nat>(
        _ctx: &mut Context<N>,
        _mutex: Mutex<M>,
        _fn: impl FnOnce(&mut Context<M>),
    )
    where N: LessThan<M> {
        todo!()
    }
}

fn test() {
    struct MainThread;

    impl thread::Thread for MainThread {
        fn run<'a>(&'a self, ctx: &'a mut thread::Context<Zero>) {
            thread::lock_and_run(ctx, thread::mutex_one(), |ctx| {
                // mutex::lock_and_run(ctx, mutex::mutex_one(), |_| {}); // not allowed
                thread::lock_and_run(ctx, thread::mutex_two(), |_ctx| {
                    todo!()
                });
            });
        }
    }

    thread::spawn(MainThread);
}
