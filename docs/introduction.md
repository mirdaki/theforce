# Introduction

> You've taken your first step into a larger world

Hello there! This document will provide an introduction to using The Force. It is going to assume some basic programming knowledge.

Some quick technical jargon: The Force is a stack based, interpreted language, with dynamic and strong typing. It's primary design goal was to optimize for number of quotes (which at times makes it cumbersome to write, but is certainly fun to read). Neither indention or newlines matter for the most part, but we encourage adding a new line between each statements and operations and indenting new scope for readability.

The Force supports these programing language constructs:
- Variables
- Printing to console
- Reading from console
- Math operations
- Logical operations
- Equality operations
- While loops
- For loops
- If/else
- Functions
- Noop

The Force supports three types:
- Floats
- Booleans (which are represented as Star Wars quotes instead of `True` or `False`)
- Strings

Operations (e.g. addition, equality) should work without any type surprises (e.g. you can add two floats, but not two strings).

To better explain the language, we'll use "keywords" that describe the function of the syntax, in addition to quotes that actually make up the language.

Files written in The Force use the `.force` extension. In addition to the snippets below, you can find some example programs in the [examples](../examples) folder.

- [Introduction](#introduction)
  - [Examples](#examples)
    - [Main](#main)
    - [Variable Declaration](#variable-declaration)
    - [Read Input](#read-input)
    - [Math Operations](#math-operations)
    - [Equality Operations](#equality-operations)
    - [Logic Operations](#logic-operations)
    - [While Loops](#while-loops)
    - [For Loops](#for-loops)
    - [If/Else](#ifelse)
    - [Void Function](#void-function)
    - [Non-Void Function](#non-void-function)
  - [Reference](#reference)

## Examples

### Main

This code prints "Hello there". There is also a comment from Yoda to help describe the code. Note: The Force executes the main function in each file. There should be no more than one.

```force
// A comment, this is
BeginMain
    Print "Hello there"
EndMain

<(-.-)> A comment, this is
Do it!
    The Sacred Jedi Texts! "Hello there"
May The Force be with you.
```

### Variable Declaration

This code creates three variables `jawa`, `ewok`, and `darkSide`, which are a float, string, and boolean respectively. Each variable is assigned an initial value and then printed.

```force
BeginMain
    DeclareFloat jawa
    SetInitialValue -13.2

    Print jawa

    DeclareString ewok
    SetInitialValue "Nub Nub"

    Print ewok

    DeclareBool darkSide
    SetInitialValue False

    Print darkSide
EndMain


Do it!
    Size matters not. jawa
    Who, mesa? -13.2

    The Sacred Jedi Texts! jawa

    Yoda. You seek Yoda. ewok
    Who, mesa? "Nub Nub"

    The Sacred Jedi Texts! ewok

    I am the Senate! darkSide
    Who, mesa? From a certain point of view.

    The Sacred Jedi Texts! darkSide
May The Force be with you.
```

### Read Input

This example creates three variables of different types and reads a value from standard in, before printing that value and continuing on to the next variable.

```force
BeginMain
    DeclareFloat jawa
    SetInitialValue 0.0

    ReadFloat jawa

    Print jawa

    DeclareString ewok
    SetInitialValue ""

    ReadString ewok

    Print ewok

    DeclareBool darkSide
    SetInitialValue True

    ReadBoolean darkSide

    Print darkSide
EndMain

Do it!
    Size matters not. jawa
    Who, mesa? 0.0

    Now this is podracing! jawa

    The Sacred Jedi Texts! jawa

    Yoda. You seek Yoda. ewok
    Who, mesa? ""

    Looking? Found someone, you have, I would say. ewok

    The Sacred Jedi Texts! ewok

    I am the Senate! darkSide
    Who, mesa? From a certain point of view.

    I hope you know what you're doing. darkSide

    The Sacred Jedi Texts! darkSide
May The Force be with you.
```

### Math Operations

This creates a variable, then performs a series of math operations before finally printing the result. Note: The initial value for this set of operations is 4, because that is the value of `porg` when it starts. Other variables or values could of been used with the `SetInitialValue` instead.

```force
BeginMain
    DeclareFloat porg
    SetInitialValue 4

    AssignVariable porg
        SetValue porg
        Add 2.0
        Subtract 1
        Multiply 3
        Divide 5
        Exponent 2
        Modulus 10
    EndAssignVariable

   Print porg
EndMain


Do it!
    Size matters not. porg
    Who, mesa? 4

    What a piece of junk! porg
        I am your father. porg
        Your lightsabers will make a fine addition to my collection. 2.0
        Proceed with the countdown. 1
        There's too many of them! 3
        Not to worry, we are still flying half a ship. 5
        Unlimited power! 2
        Never tell me the odds! 10
    The garbage will do.

    The Sacred Jedi Texts! porg
May The Force be with you.
```

### Equality Operations

This creates several float variables and compares them with each to determine which has the most of a bad idea.

```force
BeginMain
    DeclareFloat anakin
    SetInitialValue 27700

    DeclareFloat luke
    SetInitialValue 14500

    DeclareFloat leia
    SetInitialValue 14500

    DeclareBool midichlorian
    SetInitialValue False

    AssignVariable midichlorian
        SetValue luke
        GreaterThan anakin
    EndAssignVariable

   Print midichlorian

   AssignVariable midichlorian
        SetValue anakin
        LessThan leia
    EndAssignVariable

   Print midichlorian

   AssignVariable midichlorian
        SetValue leia
        Equal luke
    EndAssignVariable

   Print midichlorian
EndMain


Do it!
    Size matters not. anakin
    Who, mesa? 27700

    Size matters not. luke
    Who, mesa? 14500

    Size matters not. leia
    Who, mesa? 14500

    I am the Senate! midichlorian
    Who, mesa? That's impossible!

    What a piece of junk! midichlorian
        I am your father. luke
        Impressive. Most impressive. anakin
    The garbage will do.

    The Sacred Jedi Texts! midichlorian

    What a piece of junk! midichlorian
        I am your father. anakin
        There's always a bigger fish. leia
    The garbage will do.

    The Sacred Jedi Texts! midichlorian

    What a piece of junk! midichlorian
        I am your father. leia
        I am a Jedi, like my father before me. luke
    The garbage will do.

    The Sacred Jedi Texts! midichlorian
May The Force be with you.
```

### Logic Operations

This example creates several boolean variables and performs logical operations on them before printing the results. Note: The `Not` operator does not use a value, it just operates on the result of the last operation.

```force
BeginMain
    DeclareBool lightside
    SetInitialValue True

    DeclareBool darkside
    SetInitialValue False

    DeclareBool revan
    SetInitialValue False

    AssignVariable revan
        SetValue lightside
        Or darkside
    EndAssignVariable

   Print revan

   AssignVariable revan
        SetValue revan
        And lightside
    EndAssignVariable

   Print revan

   AssignVariable revan
        SetValue revan
        Not
    EndAssignVariable

   Print revan
EndMain


Do it!
    I am the Senate! lightside
    Who, mesa? From a certain point of view.

    I am the Senate! darkside
    Who, mesa? That's impossible!

    I am the Senate! revan
    Who, mesa? That's impossible!

    What a piece of junk! revan
        I am your father. lightside
        There is another. darkside
    The garbage will do.

    The Sacred Jedi Texts! revan

    What a piece of junk! revan
        I am your father. revan
        As you wish. lightside
    The garbage will do.

    The Sacred Jedi Texts! revan

    What a piece of junk! revan
        I am your father. revan
        Always with you it cannot be done.
    The garbage will do.

    The Sacred Jedi Texts! revan
May The Force be with you.
```

### While Loops

This example creates a float and uses it as the flag in a while loop, where each loop it prints itself and is decremented. Note: While loops continue until the flag is `0` or `False`.

```force
BeginMain
    DeclareFloat deathStars
    SetInitialValue 3

    While deathStars 
        Print deathStars
        AssignVariable deathStars
            SetValue deathStars
            Subtract 1
        EndAssignVariable
    EndWhile
EndMain


Do it!
    Size matters not. deathStars
    Who, mesa? 3

    Here we go again. deathStars
        The Sacred Jedi Texts! deathStars

        What a piece of junk! deathStars
            I am your father. deathStars
            Proceed with the countdown. 1
        The garbage will do.
    Let the past die.
May The Force be with you.
```

### For Loops

This example creates a float value and uses it as the iterator in a for loop. It then prints each new increment, in this case from 0 to 9. Note: For loops repeat until the iterator is equal to the max value set.

```force
BeginMain
    DeclareFloat deadYounglings
    SetInitialValue 0

    For 10
    ForStart deadYounglings
        The Sacred Jedi Texts! deadYounglings
    EndFor
EndMain


Do it!
    Size matters not. deadYounglings
    Who, mesa? 0

    For over a thousand generations. 10
    Let the Wookiee win. deadYounglings
        The Sacred Jedi Texts! deadYounglings
    It is clear to me now the Republic no longer functions.
May The Force be with you.
```

### If/Else

This example shows an if statement that always executes the if branch. Note: Only boolean values are accepted for the conditional.

```force
BeginMain
    If True
        Print "Do"
    ElseClause
        Print "Don't"
    EndIf 
EndMain


Do it!
    Do, or do not. There is no try. From a certain point of view.
        The Sacred Jedi Texts! "Do"
    These aren't the droids you're looking for.
        The Sacred Jedi Texts! "Don't"
    You have failed me for the last time.
May The Force be with you.
```

### Void Function

This example creates a void function that does not return a value. In this case it prints Rebel scum propaganda. The main function can then call the function and pass in a value.

```force
DeclareFunction NameTheSystem
FunctionParameters planet
Void
    Print "Goodbye "
    Print planet
    Print " *Deathstar noise*"
EndFunctionDeclaration

BeginMain
    CallFunction NameTheSystem
    PassArgument "Alderaan"
EndMain


This is where the fun begins. NameTheSystem
Now, that's a name I've not heard in a long time. A long time. planet
It's a trap!
    The Sacred Jedi Texts! "Goodbye "
    The Sacred Jedi Texts! planet
    The Sacred Jedi Texts! " *Deathstar noise*"
You cannot escape your destiny.

Do it!
    I have a bad feeling about this. NameTheSystem
    I'll try spinning, that's a good trick. "Alderaan"
May The Force be with you.
```

### Non-Void Function

This example creates a function that returns a value. In main, that value is used to set the value of `survive`, which is then printed. Note: Functions can have any number of parameters. Also, variable names are scoped, so the `survive` in main has nothing to do with the `survive` in `TheOdds`.

```force
DeclareFunction TheOdds
FunctionParameters odds
    DeclareBool survive
    SetInitialValue False

    AssignVariable survive
        SetValue odds
        Modulus 3720
    EndAssignVariable

    ReturnStatement survive
EndFunctionDeclaration

BeginMain
    DeclareBool survive
    SetInitialValue False

    AssignVariableFromFunctionCall survive
        CallFunction TheOdds
        PassArgument 42
    EndAssignVariable

    Print survive
EndMain


This is where the fun begins. TheOdds
Now, that's a name I've not heard in a long time. A long time. odds
    I am the Senate! survive
    Who, mesa? That's impossible!

    What a piece of junk! survive
        I am your father. odds
        Never tell me the odds! 3720
        I am a Jedi, like my father before me. 0
    The garbage will do.

    You're all clear, kid. Now let's blow this thing and go home. survive
You cannot escape your destiny.

Do it!
    I am the Senate! survive
    Who, mesa? That's impossible!

    Many Bothans died to bring us this information. survive
        I have a bad feeling about this. TheOdds
        I'll try spinning, that's a good trick. 52
    The garbage will do.

    The Sacred Jedi Texts! survive
May The Force be with you.
```

## Reference

| Keyword                        | Quote                                                          | Notes                                   |
| ------------------------------ | -------------------------------------------------------------- | --------------------------------------- |
| Comment                        | \|-o-\| or :><: or <(-.-)>                                     | All comments provide the same function  |
| BeginMain                      | Do it!                                                         |                                         |
| EndMain                        | May The Force be with you.                                     |                                         |
| Print                          | The Sacred Jedi Texts!                                         | Can take a variable or raw value        |
| DeclareFloat                   | Size matters not.                                              |                                         |
| DeclareString                  | Yoda. You seek Yoda.                                           |                                         |
| DeclareBool                    | I am the Senate!                                               |                                         |
| SetInitialValue                | Who, mesa?                                                     | Required after the variable declaration |
| True                           | From a certain point of view.                                  | Will also the quote for true values     |
| False                          | That's impossible!                                             | Will also the quote for false values    |
| DeclareFunction                | This is where the fun begins.                                  |                                         |
| FunctionParameters             | Now, that's a name I've not heard in a long time. A long time. |                                         |
| Void                           | It's a trap!                                                   |                                         |
| Return                         | You're all clear, kid. Now let's blow this thing and go home.  |                                         |
| EndFunctionDeclaration         | You cannot escape your destiny.                                |                                         |
| AssignVariable                 | What a piece of junk!                                          |                                         |
| AssignVariableFromFunctionCall | Many Bothans died to bring us this information.                |                                         |
| EndAssignVariable              | The garbage will do.                                           |                                         |
| SetValue                       | I am your father.                                              |                                         |
| CallFunction                   | I have a bad feeling about this.                               |                                         |
| PassArgument                   | I'll try spinning, that's a good trick.                        |                                         |
| Add                            | Your lightsabers will make a fine addition to my collection.   |                                         |
| Subtract                       | Proceed with the countdown.                                    |                                         |
| Multiply                       | There's too many of them!                                      |                                         |
| Divide                         | Not to worry, we are still flying half a ship.                 |                                         |
| Exponent                       | Unlimited power!                                               |                                         |
| Modulus                        | Never tell me the odds!                                        |                                         |
| Equal                          | I am a Jedi, like my father before me.                         | Supports floats, booleans, and strings  |
| GreaterThan                    | Impressive. Most impressive.                                   |                                         |
| LessThan                       | There's always a bigger fish.                                  |                                         |
| Not                            | Always with you it cannot be done.                             |                                         |
| Or                             | There is another.                                              |                                         |
| And                            | As you wish.                                                   |                                         |
| Noop                           | Move along. Move along.                                        | Does literally nothing                  |
| While                          | Here we go again.                                              |                                         |
| EndWhile                       | Let the past die.                                              |                                         |
| If                             | Do, or do not. There is no try.                                |                                         |
| Else                           | These aren't the droids you're looking for.                    |                                         |
| EndIf                          | You have failed me for the last time.                          |                                         |
| For                            | For over a thousand generations.                               |                                         |
| ForStart                       | Let the Wookiee win.                                           |                                         |
| EndFor                         | It is clear to me now the Republic no longer functions.        |                                         |
| ReadFloat                      | Now this is podracing!                                         | Press enter to input the value          |
| ReadString                     | Looking? Found someone, you have, I would say.                 | Press enter to input the value          |
| ReadBoolean                    | I hope you know what you're doing.                             | Press enter to input the value          |
