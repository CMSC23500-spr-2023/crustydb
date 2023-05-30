# Query Operators Milestone (qo)


CrustyDB runs sequential scans out of the box. In this milestone, you will
implement the *aggregate*, *groupby*, and *join* operators, so you can execute more
complex queries.  As in the first milestone, we provide a suite of tests that
your implementation must pass.


This milestone has been rewritten to include more guidance--hooray! We want to
offer a step-by-step guide to help with implementing these operators. The points
below are geared towards that goal.

## General Advice

- If you want to see whether your code for a function is working as intended, you can insert a panic!() statement at the end of a test for that function. This will ensure that the test fails, which means it will show any print statements. You can take advantage of those print statements to gain a better understanding of where you might be making mistakes.
- Check out the code in src/common/src/lib.rs for info on Attributes, Fields, Tuples, and other commonly-used data structures: many of them have highly useful built-in functions!
- The easiest way to get points fast is to implement the open() and close() functions and checkers in both join.rs and aggregate.rs.


## Query Operators' Logic


The very first thing that you need to have clear is how the join,
aggregate, and groupby operators work. This is independent of CrustyDB. You
should think hard what each of these operators needs to do with the data that is
fed to them for them to produce the correct output. It probably helps writing
some pseudocode on paper, sketching a quick implementation in your favorite
scripting language, or just having a clear idea of the algorithm behind each of
these operators. In general, conceptually, each logical operator is simple. In
practice, implementations can get arbitrarily complicated, depending on how much
you care about optimizing for performance, for example. In this milestone, we
are not measuring performance, just correctness.


## Execution Engine


In RDBMSs, including CrustyDB, operators do not run in a vacuum. Instead, as
we've discussed in class, they become the nodes of a query plan that is executed
by an execution engine. There are many kinds of execution engines and many ways
of implementing query plans. CrustyDB's execution engine implements a
Volcano-style interface with *open*, *next*, etc. This means that your
implementation of *aggregate*, *groupby*, and *join* will need to implement this
interface (we have a Trait, OpIterator, in the Rust-based CrustyDB implementation)
as well so it can be integrated within CrustyDB's execution engine.


Hint. Take a look at how SQL queries get parsed and translated into logical
query plans (queryexe/query/translate_and_validate.rs). Then, take a look at how
these plans are executed by studying queryexe/query/executor.rs.


## OpIterator Trait


We use Rust's Trait system to represent the Volcano-style operator interfaces.
You can find the definition in the OpIterator Trait
(queryexe/opiterator/mod.rs),
which every operator in CrustyDB implements. Furthermore, you should take a look
at the operators we have implemented for you to understand how this interface is
used in practice.


If you have set up a debugger, this is a great time to put it to use: it'll help
you trace what happens during query execution (debuggers are not only useful to
find bugs).


After you have understood the lifecycle of query execution, and once you have a
clear idea of what the *aggregate*, *groupby*, and *join* operators must do,
then it is time to implement them!


## Guide to Implementing Aggregate and Join


Unlike a sequential scan, aggregate and join operators are *stateful* and
*blocking*. They are stateful because their output depends on their input and on
some *state* that the operator must manage. They are blocking because they
cannot produce output until they have seen the entire input data. If
these two concepts seem difficult, then I encourage you to write in pseudocode
the aggregate and join operators before jumping into the real implementation.
These two ideas should be very clear in your mind!

With that established, here's a basic set of directions to help you get started:


### Step 1: Read filter.rs

Filter.rs is a file that successfully implements OpIterator.rs. It's located in
src/opiterator, next to the other Rust files for SQL commands. You’ll notice the
following components:

- FilterPredicate
- Filter
- impl OpIterator for Filter


### Step 2: Implement Join

The previously mentioned sections in filter.rs map neatly onto the JoinPredicate, Join, and impl OpIterator for the Join section of join.rs, respectively. If you're looking to get a quick few points, the open() and close() logic in particular is easy to implement.

Once you have the same logic as filter.rs, you'll probably notice that Filter is designed for dealing with one child/entry at a time... but for obvious reasons, Join needs to work on two at a time. This part is where you'll have to innovate! Note that the general structure of Filter may still help--for instance, while FilterPredicate contains a function to determine whether a tuple fits a FilterPredicate, you might want to make a similar function for JoinPredicate that determines whether two tuples fit a JoinPredicate.

### Step 3: Implement HashEqJoin

The good news is that HashEqJoin isn't all that different from Join--in fact, now that you have the underlying logic for a Join, HashEqJoin should be a lot simpler! The class lecture slides should detail the basic concept of a hash equi-join: instead of using a highly inefficient nested loop to compare every possible set of tuples, a hash equi-join does the following:

- Create a *hash table* out of one of the tables (Rust's HashMap data structure may be useful for this), where each key corresponds to a hashed ID for that tuple's relevant fields in the Join predicate, and the value is the tuple itself
- Iterate over the other table, hashing each tuple's relevant fields to generate a key. If that tuple's key is equal to any key in the hash table, then it must be a match! Join them, and then add this new value to your result.

With the two joins done, you're ready to move on to aggregate.rs.

### Step 4: merge_tuple_into_group

Don't just rush into aggregate.rs--**make a plan first!** Otherwise, you may have to restart when you realize your implementation doesn't take something into account.

Aggregate.rs is divided into these sections:
- AggregateField: An index corresponding to a column of the table, and how that column should be aggregated.
- Aggregator: A struct that contains the tricky logic required to perform an aggregation.
- Aggregate: Sort of a wrapper for Aggregator--a struct that implements OpIterator to do the actual interfacing with a user.

The bulk of the work in aggregate.rs takes place in the Aggregator function merge_tuple_into_group: Given a tuple, this function must determine:
- Whether to add it to an existing group or create a new group, based on the specified groupby_fields in Aggregator
- How the value(s) stored within the aggregation fields should be aggregated, based on the specified agg_fields in Aggregator

There are five aggregation operators: SUM, COUNT, MIN, MAX, and AVG. AVG may be the most difficult to implement because you will only be adding one tuple at a time to your calculations, so make sure to think extra about it. All 5 of these aggregation operators can be applied to ints, while COUNT, MIN, and MAX can also be applied to strings.

You'll need some way to store the groups you already have, and their aggregations. You can add any fields you want to the Aggregator struct, which may come in handy.

To help with your planning, here's a sample case to consider:

    You are running an online flower store, and have collected a database of the following customer information:

    Name    State   Variety     Price
    Alice   Alaska  Tulips      $10
    Alice   Nevada  Roses       $5
    Bob     Alaska  Tulips      $9
    Bob     Nevada  Daffodils   $7
    Bob     Nevada  Roses       $6

    1. You want to know the average order price in each state. Which groupby/aggregate statement(s) would you use?
    2. You want to know the average price of each variety of flowers. Which groupby/aggregate statement(s) would you use?
    3. You want to know the name that comes first in the alphabet for each state. Which groupby/aggregate statement(s) would you use?
    4. You want to know the total sum of how much each person has spent on flowers, and how many orders they've placed. (Assume each name/state combo is a unique person.) Which groupby/aggregate statement(s) would you use?
    
I would suggest thinking through these cases, or writing them down. What fields will you need in your struct to handle all the cases? What assumptions are you making? Remember: you can have multiple groupby fields *and* multiple aggregate fields in the same call!

### Step 5: Bringing it Together

Once your merge_tuple_into_group function is written, you'll start passing a lot more autograder tests! Yay! Now, you're in the home stretch--your experiences in join.rs have taught you how to implement OpIterator, and Iterator shouldn't be too hard. Make sure to look at tuple_iterator.rs if you need help with creating a TupleIterator object!


## End-To-End (e2e) Testing
We have given a new directory/project for automating e2e testing of queries.
This project allows for a test file to be specified that can list a series of
commands and queries, with expected results. More details are coming soon on these
tests.  You can run all tests by the following command


```
cd e2e-tests
sh runTests.sh
```


This will execute every test in e2e-tests/testdata.
Note the test `run_sql_logic_tests` attempts to run all of the tests in the testdata directory.
The test `run_sql_join` is redundant, and runs a single test (which is covered by the full test suite).
We include this so you can see how to run one test if needed for debugging.
You should also note that these tests work by running a client and server. If something goes wrong and
your server does not properly close/clean-up/reset you may leave a server process hung (running in
in the background.) When you try to run a test again you will see the port is blocked. You will need to identify
the process and kill it (it will likely be called server).  You might need to search for how to do this
if you haven't. For example, this SO article shows how to do this based on a port for Mac:
https://stackoverflow.com/questions/3855127/find-and-kill-process-locking-port-3000-on-mac .


## Scoring and Requirements


70% of your score on this milestone is based on correctness that is demonstrated
by passing all of the provided unit and integration tests in the queryexe crate.
This means when running `cargo test -p queryexe` all tests pass.
10% of your score is based on whether we can run queries that include
aggregates, groupby, and joins end to end (so you should make sure this is
possible and you may want to write additional tests to harden your
implementation). 10% is based on code quality. 10% is based on your write
up (my-op.txt). The write up should contain:


-  A brief description of your solution. In particular, include what design
decisions you made and explain why. This is only needed for those parts of your
solutions that had some significant design work (e.g. how you implemented and handled
state in the aggregate and join operators).


- How long you roughly spent on the milestone, and what would have
liked/disliked on the milestone.


- If you know some part of the milestone is incomplete, write up what parts are
not working, how close you think you are, and what part(s) you got stuck on.


## Additional Tips
If you run `cargo test -p queryexe` before implementing anything, you will see 33 passed tests. This is because several operators have been implemented. Your goal is to implement aggregate and join to pass the rest of the test cases without breaking other passed tests. If you want to run test cases under a particular operator - use `cargo test -p queryexe opiterator::<op to test>` , e.g. `cargo test -p queryexe opiterator::join`




## 33550


For students in 33550 you will need to add one open-ended component that will count for 25% (coming from the 70% correctness portion).
*You may opt to not complete this step for the score penalty.*  You will need to integrate the component into your code,
provide tests to test the code (likely unit, integration, and end-to-end) and detail in the write up how to invoke the tests.
Some ideas are given below and their expected difficulty.  For easier tasks we will expect a more thorough evaluation (performance and/or correctness).


Many of these will require integrating with the update OpIterator (for record updates), mutator (for inserts), or transaction_manager and may
require changing some existing API calls in Crusty.


Recommended Projects:


- Integrate hash or tree based index using std data structures into query execution and develop a simple query optimizer to choose when to use the query execution (medium).
- Develop statistic collection as records are loaded or updated and a method to store samples for basic operations estimation (selectivity of a predicate). Invoking the statistical estimate may require a new command (e.g. \stats <table_name> <operation>). (easy to medium)
- Support a grace hash join writing the partitions to disk via the storage manager (easy to medium)
- Parallelize a simple hash join operator and hash-based aggregation operator (easy to medium)
- Implement a simple write-ahead log for updates (easy to medium)
- Developing new test suite for concurrent evaluation of HS (both for correctness and performance). (medium)


Ambitious Projects:
- Support nested queries (medium to hard)
- Develop a hash or tree index from scratch and have it not integrated but tested (medium to hard)
- Implement a basic query optimizer for join ordering (medium)
- Implement a 2PL transaction manager (hard)
