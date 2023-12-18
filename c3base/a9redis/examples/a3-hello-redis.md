在这里，我们创建了 N 个不同的存储实例，每个实例都会存储不同的分片数据，例如我们有a-i共 9 个不同的 key, 可以将存储分成 3
个实例，那么第一个实例可以存储 a-c，第二个d-f，以此类推。在这种情况下，访问 b 时，只需要锁住第一个实例，此时二、三实例依然可以正常访问，因此锁被成功的分片了。

在分片后，使用给定的 key 找到对应的值就变成了两个步骤：首先，使用 key 通过特定的算法寻找到对应的分片，然后再使用该 key
从分片中查询到值:

let shard = db[hash(key) % db.len()].lock().unwrap();
shard.insert(key, value);
这里我们使用 hash 算法来进行分片，但是该算法有个缺陷：分片的数量不能变，一旦变了后，那之前落入分片 1
的key很可能将落入到其它分片中，最终全部乱掉。此时你可以考虑dashmap，它提供了更复杂、更精妙的支持分片的hash map。

https://github.com/xacrimon/dashmap

在 .await 期间持有锁
在某些时候，你可能会不经意写下这种代码:

```rust

use std::sync::{Mutex, MutexGuard};

async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
    *lock += 1;

    do_something_async().await;
} // 锁在这里超出作用域
```

如果你要 spawn 一个任务来执行上面的函数的话，会报错:
错误的原因在于 std::sync::MutexGuard 类型并没有实现 Send 特征，这意味着你不能将一个 Mutex 锁发送到另一个线程，因为 .await
可能会让任务转移到另一个线程上执行，这个之前也介绍过。

提前释放锁
要解决这个问题，就必须重构代码，让 Mutex 锁在 .await 被调用前就被释放掉。

```rust

// 下面的代码可以工作！
async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    {
        let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
        *lock += 1;
    } // lock在这里超出作用域 (被释放)

    do_something_async().await;
}
```

大家可能已经发现，很多错误都是因为 .await 引起的，其实你只要记住，在 .await 执行期间，任务可能会在线程间转移，那么这些错误将变得很好理解，不必去死记硬背

但是下面的代码不工作：

```rust

use std::sync::{Mutex, MutexGuard};

async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
    *lock += 1;
    drop(lock);

    do_something_async().await;
}
```

原因我们之前解释过，编译器在这里不够聪明，目前它只能根据作用域的范围来判断，drop
虽然释放了锁，但是锁的作用域依然会持续到函数的结束，未来也许编译器会改进，但是现在至少还是不行的。

聪明的读者此时的小脑袋已经飞速运转起来，既然锁没有实现 Send，
那我们主动给它实现如何？这样不就可以顺利运行了吗？答案依然是不可以，原因就是我们之前提到过的死锁，如果一个任务获取了锁，然后还没释放就在
.await 期间被挂起，接着开始执行另一个任务，这个任务又去获取锁，就会导致死锁。

再来看看其它解决方法：

重构代码：在 .await 期间不持有锁
之前的代码其实也是为了在 .await 期间不持有锁，但是我们还有更好的实现方式，例如，你可以把 Mutex
放入一个结构体中，并且只在该结构体的非异步方法中使用该锁:

```rust

use std::sync::Mutex;

struct CanIncrement {
    mutex: Mutex<i32>,
}

impl CanIncrement {
    // 该方法不是 `async`
    fn increment(&self) {
        let mut lock = self.mutex.lock().unwrap();
        *lock += 1;
    }
}

async fn increment_and_do_stuff(can_incr: &CanIncrement) {
    can_incr.increment();
    do_something_async().await;
}
```

使用异步任务和通过消息传递来管理状态

