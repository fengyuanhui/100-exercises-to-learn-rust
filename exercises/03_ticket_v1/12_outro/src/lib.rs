// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

/**
 * 订单
 */
pub struct Order {
    // 产品名称
    product_name: String,
    // 数量
    quantity: u32,
    // 单价
    unit_price: u16,
}

impl Order {
    /**
     * 产品名称校验
     */
    fn validate_product_name(product_name: &str) {
        if product_name.is_empty() {
            panic!("产品名称不能为空");
        }

        if product_name.len() > 300 {
            panic!("产品名称字节长度不能超过300");
        }
    }

    /**
     * 数量校验
     */
    fn validate_quantity(quantity: &u32) {
        if quantity <= &0 {
            panic!("数量必须大于 0 ");
        }
    }

    /**
     * 单价校验
     */
    fn validate_unit_price(unit_price: &u16) {
        if unit_price <= &0 {
            panic!("单价必须大于0");
        }
    }

    /**
     * 构造函数
     */
    pub fn new(product_name: String, quantity: u32, unit_price: u16) -> Self {
        Order::validate_product_name(&product_name);
        Order::validate_quantity(&quantity);
        Order::validate_unit_price(&unit_price);

        Self {
            product_name,
            quantity,
            unit_price,
        }
    }

    /**
     * 获取产品名称
     */
    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    /**
     * 设置产品名称
     */
    pub fn set_product_name(&mut self, product_name: String) {
        Order::validate_product_name(&product_name);
        self.product_name = product_name;
    }

    /**
     * 获取数量
     */
    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    /**
     * 设置数量
     */
    pub fn set_quantity(&mut self, quantity: u32) {
        Order::validate_quantity(&quantity);
        self.quantity = quantity;
    }

    /**
     * 获取单价
     */
    pub fn unit_price(&self) -> &u16 {
        &self.unit_price
    }

    /**
     * 设置单价
     */
    pub fn set_unit_price(&mut self, unit_price: u16) {
        Order::validate_unit_price(&unit_price);
        self.unit_price = unit_price;
    }

    /**
     * 计算订单总费用
     */
    pub fn total(&self) -> u32 {
        self.quantity * (self.unit_price as u32)
    }
}
