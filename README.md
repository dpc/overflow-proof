# Monadic checked arithmetic for Rust

This library provides types wrapping raw numeric types,
and tracking possibility of an overflow, enforcing
correct handling without possibility of panics or
incorrect values (when overflow checks in release build
are disabled).

[`Checked`] is the main type provided by this library.

```rust
use overflow_proof::Checked;

let a = Checked::new(2u8);
let b = Checked::new(100u8);

// Aritmetic operations can be chained like with normal types
assert!({ ((a + 2) / 3 + 5) * b + 1}.check().is_none());
assert_eq!(*{ a + 2 + b }.check().expect("overflow"), 104);
```

```rust
use overflow_proof::{Checked, WithDeref};

struct OverflowError;

struct BankAccount {
  balance: Checked<u64>,
}

impl BankAccount {
  fn debit(&mut self, amount: u64) -> Result<(), OverflowError> {
  // Will not compile:
  // Ok(self.balance -= amount)

  // Overflow must be checked:
  Ok(self.balance = {self.balance - amount}.check().ok_or(OverflowError)?)
  }
}
```
