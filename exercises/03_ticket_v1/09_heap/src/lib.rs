pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        // String 在 Rust 中是一个包含三个字段的结构体，每个字段的大小与平台的指针宽度一致：
        // 指针 (ptr)：指向堆上存储实际字符串数据的指针。
        // 长度 (len)：当前字符串的字节长度。
        // 容量 (capacity)：堆上已分配的总字节容量。

        // 在 64 位系统上，指针和 usize 都是 8 字节，因此 String 结构体本身的大小为 8 * 3 = 24 字节
        // 在 32 位系统上，指针和 usize 都是 4 字节，因此 String 结构体本身的大小为 4 * 3 = 12 字节
        assert_eq!(size_of::<String>(), 24);
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Type layout" section of The Rust Reference
        // https://doc.rust-lang.org/reference/type-layout.html for more information.

        // 在 64 位系统上，size_of::<Ticket>() 返回 72 字节；在 32 位系统上返回 36 字节。
        // 计算过程非常简单：
        // Ticket 结构体包含 3 个 String 类型的字段。
        // 如上一问所述，在 64 位系统上每个 String 占 24 字节，在 32 位系统上每个占 12 字节。
        // 因为三个字段类型完全相同，对齐要求也一致（64 位下为 8 字节对齐，32 位下为 4 字节对齐），字段之间不需要额外的填充字节。
        // 所以计算公式为：
        // 64位：24 字节 × 3 = 72 字节
        // 32位：12 字节 × 3 = 36 字节
        assert_eq!(size_of::<Ticket>(), 72);
    }
}
