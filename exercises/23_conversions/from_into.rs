// The `From` trait is used for value-to-value conversions. If `From` is
// implemented, an implementation of `Into` is automatically provided.
// You can read more about it in the documentation:
// https://doc.rust-lang.org/std/convert/trait.From.html
// 'From' 特征用于值到值的转换。如果 'From' 是
// 实现后，将自动提供 “into” 的实现。
// 您可以在文档中阅读更多信息:
// https:// doc.rust-lang.org/std/convert/trait.From.html

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 我们实现 Default 特征，以便在提供的字符串无法转换为 `Person` 对象时使用它作为后备。
// We implement the Default trait to use it as a fallback when the provided
// string is not convertible into a `Person` object.
impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("John"),
            age: 30,
        }
    }
}

// TODO: Complete this `From` implementation to be able to parse a `Person`
// out of a string in the form of "Mark,20".
// Note that you'll need to parse the age component into a `u8` with something
// like `"4".parse::<u8>()`.
//
// Steps:
// 1. Split the given string on the commas present in it.
// 2. If the split operation returns less or more than 2 elements, return the
//    default of `Person`.
// 3. Use the first element from the split operation as the name.
// 4. If the name is empty, return the default of `Person`.
// 5. Parse the second element from the split operation into a `u8` as the age.
// 6. If parsing the age fails, return the default of `Person`.
// TODO: 完成这个 'From' 实现，以便能够解析一个 'Person'
// 以 “Mark，20” 的形式输出一个字符串。
// 请注意，您需要将age组件解析为带有某些内容的 “u8”
// like '“4”.parse ::< u8>()'。
//
// 步骤:
// 1.拆分给定的字符串中存在的逗号。
// 2.如果拆分操作返回少于或多于2个元素，则返回
// “Person” 的默认值。
// 3.使用拆分操作中的第一个元素作为名称。
// 4.如果名称为空，则返回默认值 “person”。
// 5.将拆分操作中的第二个元素解析为 “u8” 作为年龄。
// 6.如果解析年龄失败，则返回默认值 “person”。
impl From<&str> for Person {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Default::default();
        }
        let name = parts[0].to_string();
        if name.is_empty() {
            return Default::default();
        }
        let age = parts[1].parse::<u8>().unwrap_or(0);
        if age == 0 {
            return Default::default();
        }
        Person { name, age }
    }
}

fn main() {
    // Use the `from` function.
    let p1 = Person::from("Mark,20");
    println!("{p1:?}");

    // Since `From` is implemented for Person, we are able to use `Into`.
    let p2: Person = "Gerald,70".into();
    println!("{p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn test_bad_convert() {
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_good_convert() {
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }

    #[test]
    fn test_bad_age() {
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,dog");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
