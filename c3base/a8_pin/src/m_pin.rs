use std::marker::PhantomPinned;
use std::pin::Pin;

#[derive(Debug)]
struct Test {
  data: String,
  r_data: *const String, // rData 是一个裸指针，引用了data的值
  _marker: PhantomPinned
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_d1() {
    let str = String::from("数据");
    let mut test = Test {
      data: str, // 1.这里str的所有权转移到了test.data，所以rData赋值为&str会抛出所有权错误
      r_data: std::ptr::null(), // 2. 所以这里，我们先初始化一个空指针
      _marker: PhantomPinned
    };
    // println!("init test: {:?} data: {}, rData: {}", test, test.data, unsafe { &*test.r_data });

    // 3. 获取 test.data 的引用然后再设置给 rData
    test.r_data = &test.data;
    println!("test: {:?} data: {}, rData: {}, data address: {:p}", test, test.data, unsafe { &*test.r_data }, &test.data);
    // print data memory address
    println!("data address: {:p}", &test.data);

    // 如果把data的值改变了呢
    test.data = String::from("应用");
    println!("data address: {:p}", &test.data);
   
    // 4. 这里我们再次打印一下test，看看rData的值是否改变了，还是没有变
    let t2 = test;
    println!("test: {:?} data: {}, rData: {}, data address: {:p}", t2, t2.data, unsafe { &*t2.r_data }, &t2.data);
    print_test(t2)
  }
  fn print_test(test: Test) {
    println!("test: {:?} data: {}, rData: {}, data address: {:p}", test, test.data, unsafe { &*test.r_data }, &test.data);
  }

  #[test]
  fn test_pin() {
    let str = String::from("数据");
    let mut pin_test = Box::pin(Test {
      data: str,
      r_data: std::ptr::null(),
      _marker: PhantomPinned
    });
    unsafe {
      pin_test.as_mut().get_unchecked_mut().r_data = &pin_test.as_ref().data;
    }
    println!("pin_test: {:?} data: {}, rData: {}", pin_test, pin_test.data, unsafe { &*pin_test.r_data });
    unsafe {
      pin_test.as_mut().get_unchecked_mut().data = String::from("应用");
    }
    println!("pin_test: {:?} data: {}, rData: {}", pin_test, pin_test.data, unsafe { &*pin_test.r_data });

  }
}