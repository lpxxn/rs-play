


编译器使用三条消除规则来确定哪些场景不需要显式地去标注生命周期。其中第一条规则应用在输入生命周期上，第二、三条应用在输出生命周期上。若编译器发现三条规则都不适用时，就会报错，提示你需要手动标注生命周期
1. 每一个引用参数都会获得独自的生命周期
例如一个引用参数的函数就有一个生命周期标注: `fn foo<'a>(x: &'a i32)`，两个引用参数的有两个生命周期标注:`fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`, 依此类推。
2. 若只有一个输入生命周期(函数参数中只有一个引用类型)，那么该生命周期会被赋给所有的输出生命周期，也就是所有返回值的生命周期都等于该输入生命周期
例如函数 `fn foo(x: &i32) -> &i32`，x 参数的生命周期会被自动赋给返回值 &i32，因此该函数等同于 `fn foo<'a>(x: &'a i32) -> &'a i32`
3. 若存在多个输入生命周期，且其中一个是 &self 或 &mut self，则 &self 的生命周期被赋给所有的输出生命周期
拥有 &self 形式的参数，说明该函数是一个 方法，该规则让方法的使用便利度大幅提升。
规则其实很好理解，但是，爱思考的读者肯定要发问了，例如第三条规则，若一个方法，它的返回值的生命周期就是跟参数 &self 的不一样怎么办？总不能强迫我返回的值总是和 &self 活得一样久吧？! 问得好，答案很简单：手动标注生命周期，因为这些规则只是编译器发现你没有标注生命周期时默认去使用的，当你标注生命周期后，编译器自然会乖乖听你的话。
   让我们假装自己是编译器，然后看下以下的函数该如何应用这些规则：

例子 1

fn first_word(s: &str) -> &str { // 实际项目中的手写代码
首先，我们手写的代码如上所示时，编译器会先应用第一条规则，为每个参数标注一个生命周期：

fn first_word<'a>(s: &'a str) -> &str { // 编译器自动为参数添加生命周期
此时，第二条规则就可以进行应用，因为函数只有一个输入生命周期，因此该生命周期会被赋予所有的输出生命周期：

fn first_word<'a>(s: &'a str) -> &'a str { // 编译器自动为返回值添加生命周期
此时，编译器为函数签名中的所有引用都自动添加了具体的生命周期，因此编译通过，且用户无需手动去标注生命周期，只要按照 fn first_word(s: &str) -> &str { 的形式写代码即可。

例子 2 再来看一个例子：

fn longest(x: &str, y: &str) -> &str { // 实际项目中的手写代码
首先，编译器会应用第一条规则，为每个参数都标注生命周期：

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
但是此时，第二条规则却无法被使用，因为输入生命周期有两个，第三条规则也不符合，因为它是函数，不是方法，因此没有 &self 参数。在套用所有规则后，编译器依然无法为返回值标注合适的生命周期，因此，编译器就会报错，提示我们需要手动标注生命周期：

方法中的生命周期
先来回忆下泛型的语法：

struct Point<T> {
x: T,
y: T,
}

impl<T> Point<T> {
fn x(&self) -> &T {
&self.x
}
}
实际上，为具有生命周期的结构体实现方法时，我们使用的语法跟泛型参数语法很相似：

struct ImportantExcerpt<'a> {
part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
fn level(&self) -> i32 {
3
}
}
其中有几点需要注意的：

impl 中必须使用结构体的完整名称，包括 <'a>，因为生命周期标注也是结构体类型的一部分！
方法签名中，往往不需要标注生命周期，得益于生命周期消除的第一和第三规则
下面的例子展示了第三规则应用的场景：

impl<'a> ImportantExcerpt<'a> {
fn announce_and_return_part(&self, announcement: &str) -> &str {
println!("Attention please: {}", announcement);
self.part
}
}
首先，编译器应用第一规则，给予每个输入参数一个生命周期:

impl<'a> ImportantExcerpt<'a> {
fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &str {
println!("Attention please: {}", announcement);
self.part
}
}
需要注意的是，编译器不知道 announcement 的生命周期到底多长，因此它无法简单的给予它生命周期 'a，而是重新声明了一个全新的生命周期 'b。

接着，编译器应用第三规则，将 &self 的生命周期赋给返回值 &str：

impl<'a> ImportantExcerpt<'a> {
fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'a str {
println!("Attention please: {}", announcement);
self.part
}
}
Bingo，最开始的代码，尽管我们没有给方法标注生命周期，但是在第一和第三规则的配合下，编译器依然完美的为我们亮起了绿灯。

在结束这块儿内容之前，再来做一个有趣的修改，将方法返回的生命周期改为'b：

impl<'a> ImportantExcerpt<'a> {
fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str {
println!("Attention please: {}", announcement);
self.part
}
}
此时，编译器会报错，因为编译器无法知道 'a 和 'b 的关系。 &self 生命周期是 'a，那么 self.part 的生命周期也是 'a，但是好巧不巧的是，我们手动为返回值 self.part 标注了生命周期 'b，因此编译器需要知道 'a 和 'b 的关系。

有一点很容易推理出来：由于 &'a self 是被引用的一方，因此引用它的 &'b str 必须要活得比它短，否则会出现悬垂引用。因此说明生命周期 'b 必须要比 'a 小，只要满足了这一点，编译器就不会再报错：

impl<'a: 'b, 'b> ImportantExcerpt<'a> {
fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
println!("Attention please: {}", announcement);
self.part
}
}
Bang，一个复杂的玩意儿被甩到了你面前，就问怕不怕？

就关键点稍微解释下：

'a: 'b，是生命周期约束语法，跟泛型约束非常相似，用于说明 'a 必须比 'b 活得久
可以把 'a 和 'b 都在同一个地方声明（如上），或者分开声明但通过 where 'a: 'b 约束生命周期关系，如下：
impl<'a> ImportantExcerpt<'a> {
fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str
where
'a: 'b,
{
println!("Attention please: {}", announcement);
self.part
}
}
总之，实现方法比想象中简单：加一个约束，就能暗示编译器，尽管引用吧，反正我想引用的内容比我活得久，爱咋咋地，我怎么都不会引用到无效的内容！



