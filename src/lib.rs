use pgx::prelude::*;

pgx::pg_module_magic!();

/// returns the sum of array 
#[ pg_extern ]
fn sum_array( input : Vec< i32 > ) -> i64 
{

    let mut sum = 0 as i64;

    for i in input 
    {
        sum += i as i64;
    }
    sum

}

/// changes all values of array and returns array with modified values
#[ pg_extern ]
fn index_array( index : f64, input : Vec< f64 > ) -> Vec< f64 > 
{

    let output = input.iter().map( | x | x * index ).collect();
    output

}

#[ cfg( any( test, feature = "pg_test" ) ) ]
#[ pg_schema ]
mod tests 
{

    use pgx::prelude::*;

    #[ pg_test ]
    fn test_sum_money() 
    {

        let new_vec = vec![ 100, 200, 300 ];
        let expected = 600;
        assert_eq!( expected, crate::sum_array( new_vec ) );

    }

    #[ pg_test ]
    fn test_index_money() 
    {

        let new_vec = vec![ 100.0, 200.0, 300.0 ];
        let expected = vec![ 50.0, 100.0, 150.0 ];
        assert_eq!( expected, crate::index_array( 0.5, new_vec ) );

    }

}

/// This module is required by `cargo pgx test` invocations. 
/// It must be visible at the root of your extension crate.
#[ cfg( test ) ]
pub mod pg_test 
{

    pub fn setup( _options : Vec< &str > ) 
    {

        // perform one-off initialization when the pg_test framework starts

    }

    pub fn postgresql_conf_options() -> Vec< &'static str > 
    {

        // return any postgresql.conf settings that are required for your tests
        vec![]

    }

}
