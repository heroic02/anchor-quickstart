use anchor_lang::prelude::*;

declare_id!("5joPGj3ZQsqTTU1eP8Yc7o8aPGb5p7iivFEGJFfuCAYv");

// *** CONSTANT DECLARED HERE ***
const OWNER: &str = "8os8PKYmeVjU1mmwHZZNTEv5hpBXi5VvEKGzykduZAik";
const MEANING_OF_LIFE_AND_EXISTENCE: u64 = 42;

#[program]
pub mod learn {
    use super::*;
    use std::collections::HashMap;

    pub fn initialize(ctx: Context<Initialize>, key: String, value: String) -> Result<()> {
        let mut my_map = HashMap::new();
        my_map.insert(key.to_string(), value.to_string());
        msg!("My name is {} ", my_map[&key]);
        Ok(())
    }

    pub fn log_contanst(ctx: Context<Initialize>) -> Result<()> {
        msg!(&format!(
            "Answer to the ultimate question: {}",
            MEANING_OF_LIFE_AND_EXISTENCE
        ));
        Ok(())
    }

    pub fn usize_example(ctx: Context<Initialize>) -> Result<()> {
        let mut dynamic_array: Vec<u32> = Vec::from([1, 2, 3, 4, 5, 6]);
        let len = dynamic_array.len();
        let another_var: u64 = 5;
        let len_plus_another_var = len as u64 + another_var;
        msg!("The result is {}", len_plus_another_var);
        Ok(())
    }

    pub fn to_can_log_the_struct(ctx: Context<Initialize>) -> Result<()> {
        #[derive(Debug)]
        struct MyValues<T> {
            foo: T,
        }
        let first_struct: MyValues<i32> = MyValues { foo: 1 };
        let second_struct: MyValues<bool> = MyValues { foo: false };
        msg!("{:?}", first_struct);
        msg!("{:?}", second_struct);

        Ok(())
    }

    pub fn fn_struct(ctx: Context<Initialize>, name: String, age: u64) -> Result<()> {
        struct Person {
            name: String,
            age: u64,
        }

        let mut person1: Person = Person { name, age };

        msg!("{} is {} years old", person1.name, person1.age);

        person1.name = "Bob".to_string();
        person1.age = 18;
        msg!("{} is {} years old", person1.name, person1.age);
        Ok(())
    }

    pub fn say_hello(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello world");
        Ok(())
    }

    pub fn add(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let diffrence = a - b;
        msg!("Difference is {}", diffrence);
        Ok(())
    }

    pub fn limit_range(ctx: Context<LimitRange>, a: u64) -> Result<()> {
        // if a < 10 {
        //     return err!(MyError::AisTooSmall);
        // }
        // if a > 100 {
        //     return err!(MyError::AisTooBig);
        // }
        require!(a >= 10, MyError::AisTooSmall);
        require!(a <= 100, MyError::AisTooBig);
        msg!("Result = {}", a);
        Ok(())
    }
    pub fn func(ctx: Context<LimitRange>) -> Result<()> {
        msg!("Will this print?");
        return err!(MyError::AlwayErrors);
    }

    pub fn age_checker(ctx: Context<Initialize>, age: u64) -> Result<()> {
        // let result = if  age>=18 {"You are 18 or above"} else {"you are below 18"};
        // msg!("{:?}",result);

        match age {
            1 => {
                msg!("The age is 1");
            }
            2 | 3 => {
                msg!("The age is either 2 or 3");
            }
            4..=6 => msg!("The age is between 4 and 6"),
            _ => msg!("The age is something else"),
        }
        Ok(())
    }

    pub fn for_loop(ctx: Context<Initialize>) -> Result<()> {
        // 1
        for i in 0..10 {
            msg!("run number {}", i)
        }

        // 2
        for i in (0..10).step_by(2) {
            msg!("run number {}", i)
        }

        Ok(())
    }

    pub fn fixed_array(ctx: Context<Initialize>) -> Result<()> {
        let my_array: [u32; 5] = [10, 20, 30, 40, 50];
        let first_element = my_array[0];
        let third_element = my_array[2];

        let mut mutable_array: [u32; 3] = [10, 20, 30];
        mutable_array[1] = 250;
        msg!("mutable_array: {:?}", mutable_array);
        Ok(())
    }

    pub fn dynamic_array(ctx: Context<Initialize>) -> Result<()> {
        let mut dynamic_array: Vec<u32> = Vec::new();
        dynamic_array.push(10);
        dynamic_array.push(20);
        dynamic_array.push(30);
        let first_element = dynamic_array[0];
        let third_element = dynamic_array[2];
        msg!("Third element = {}", third_element);
        Ok(())
    }

    // Options, Enums, and Deref*
    pub fn test_option(ctx: Context<Initialize>) -> Result<()> {
        let v = Vec::from([1, 2, 3, 4, 5]);
        // Because v.iter().max() will return Option type so let's use unwrap
        assert!(*v.iter().max().unwrap() == 5);
        Ok(())
    }

    pub fn encode_and_decode(_ctx: Context<Initialize>) -> Result<()> {
        // Create a new instance of the `Person` struct
        let init_person: Person = Person {
            name: "Alice".to_string(),
            age: 27,
        };

        // Encode the `init_person` struct into a byte vector
        let encoded_data: Vec<u8> = init_person.try_to_vec().unwrap();

        // Decode the encoded data back into a `Person` struct
        let data: Person = decode(_ctx, encoded_data)?;

        // Logs the decoded person's name and age
        msg!("My name is {:?}, I am {:?} years old.", data.name, data.age);

        Ok(())
    }

    pub fn decode(_accounts: Context<Initialize>, encoded_data: Vec<u8>) -> Result<Person> {
        // Decode the encoded data back into a `Person` struct
        let decoded_data: Person = Person::try_from_slice(&encoded_data).unwrap();

        Ok(decoded_data)
    }

    pub fn create_person(ctx: Context<Initialize>) -> Result<Person> {
        let person: Person = Person::new("Hungdv".to_string(), 21);
        msg!("{:?} can drink", person.can_drink());
        msg!("{:?} add more one year", person.age_in_one_year());
        msg!("{:?} name", person.name);
        Ok(person)
    }
    pub fn fn_trait(ctx: Context<Initialize>) -> Result<()> {
        let car = Car { speed_mph: 60.0 };
        let boat = Boat { speed_knots: 30.0 };

        let car_speed_kph = car.get_speed_kph();
        let boat_speed_kph = boat.get_speed_kph();
        msg!("Car speed: {} km/h", car_speed_kph);
        msg!("Boat speed: {} km/h", boat_speed_kph);
        Ok(())
    }

    // this is private function
    fn get_a_num() -> u64 {
        2
    }

    pub fn test_clock(ctx: Context<Initialize>) -> Result<()> {
        let clock: Clock = Clock::get()?;
        msg!("Block timestamp: {} ", clock.unix_timestamp);
        emit!(MyEvent { value: 42 });
        Ok(())
    }

    #[access_control(check(&ctx))]
    pub fn test_only_owner(ctx: Context<OnlyOwner>) -> Result<()> {
        msg!("Hello, Im the owner");
        Ok(())
    }

    fn check(ctx: &Context<OnlyOwner>) -> Result<()> {
        // Check if signer === owner
        require_keys_eq!(
            ctx.accounts.signer_account.key(),
            OWNER.parse::<Pubkey>().unwrap(),
            OnlyOwnerError::NotOwner
        );
        Ok(())
    }
    pub fn read_balance(ctx: Context<ReadBalance>) -> Result<()> {
        let balance = ctx.accounts.acct.to_account_info().lamports();

        msg!("balance in Lamports is {}", balance);
        Ok(())
    }

    pub fn send_sol(ctx: Context<SendSol>, amount: u64) -> Result<()> {
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.recipient.to_account_info(),
            },
        );

        let res = system_program::transfer(cpi_context, amount);

        if res.is_ok() {
            return Ok(());
        } else {
            return err!(Errors::TransferFailed);
        }
    }
}

#[error_code]
pub enum Errors {
    #[msg("transfer failed")]
    TransferFailed,
}

#[derive(Accounts)]
pub struct SendSol<'info> {
    /// CHECK: we do not read or write the data of this account
    #[account(mut)]
    recipient: UncheckedAccount<'info>,

    system_program: Program<'info, System>,

    #[account(mut)]
    signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct ReadBalance<'info> {
    /// CHECK: although we read this account's balance, we don't do anything with the information
    pub acct: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct AccountInitialize<'info> {
    #[account(init,
              payer = signer,
              space=size_of::<MyStorage>() + 8,
              seeds = [],
              bump)]
    pub my_storage: Account<'info, MyStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyStorage {
    x: u64,
}

#[derive(Accounts)]
pub struct Initialize {}

#[error_code]
pub enum MyError {
    #[msg("a is too big")]
    AisTooBig,
    #[msg("a is too small")]
    AisTooSmall,
    #[msg("Always errors")]
    AlwayErrors,
}

#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call this function")]
    NotOwner,
}

#[derive(Accounts)]
pub struct LimitRange {}

#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    signer_account: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person { name, age }
    }
    fn can_drink(&self) -> bool {
        if self.age >= 21 as u8 {
            return true;
        }
        return false;
    }
    fn age_in_one_year(&self) -> u8 {
        return &self.age + 1;
    }
}
#[event]
pub struct MyEvent {
    pub value: u64,
}

trait Speed {
    fn get_speed_kph(&self) -> f64;
}

struct Car {
    speed_mph: f64,
}

struct Boat {
    speed_knots: f64,
}

// Traits are implemented for a type using the `impl` keyword as shown below
impl Speed for Car {
    fn get_speed_kph(&self) -> f64 {
        self.speed_mph * 1.60934
    }
}

impl Speed for Boat {
    fn get_speed_kph(&self) -> f64 {
        self.speed_knots * 1.852
    }
}
