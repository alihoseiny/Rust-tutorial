///
/// ```compile_fail
/// let a = b = 10;
/// ```
fn test1(){}

///
/// ```compile_fail
/// let a = (let b = 10);
/// ```
fn test2(){}