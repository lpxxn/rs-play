#[cfg(test)]
mod test_enum2 {
    use crate::a2_enum::test_enum2::List::Nil;

    enum List {
        // Cons: 链表中包含有值的节点，节点是元组类型，第一个元素是节点的值，第二个节点是指向下一个节点的指针
        Cons(u32, Box<List>),
        // Nil: 链表中的最后一个节点，用于说明链表的结束
        Nil,
    }

    impl List {
        // 创建空的链表
        fn new() -> List {
            Nil
        }

        // 在老的链表前面新增一个节点，并返回新的链表
        fn prepend(self, elem: u32) -> List {
            List::Cons(elem, Box::new(self))
        }

        // 返回链表长度
        fn len(&self) -> u32 {
            match *self {
                // 不能拿走 tail 的所有权，所以需要使用 tail 的引用
                List::Cons(_, ref tail) => 1 + tail.len(),
                // 空链表长度为 0
                Nil => 0,
            }
        }

        // 返回链表的字符串表现形式
        fn stringify(&self) -> String {
            match *self {
                List::Cons(head, ref tail) => {
                    // format! 和 print! 类似，但是返回的是一个堆分配的字符串，而不是打印结果到控制台上
                    format!("head: {}, tail: {}", head, tail.stringify())
                }
                List::Nil => {
                    format!("Nil")
                }
            }
        }
    }

    #[test]
    fn test_list() {
        let mut list = List::new();
        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);
        println!("linked list has length: {}", list.len());
        println!("{}", list.stringify());
    }
}