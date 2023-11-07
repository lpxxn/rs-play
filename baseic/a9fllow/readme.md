
| 使用方法	                         | 等价使用方式	                                            |所有权|
|-------------------------------|----------------------------------------------------|---|
| for item in collection	       | for item in IntoIterator::into_iter(collection)	   |转移所有权|
| for item in &collection	      | for item in collection.iter()                      |	不可变借用|
| for item in &mut collection	  | for item in collection.iter_mut()                  |	可变借用|
