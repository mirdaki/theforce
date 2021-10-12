# Design of the Force

WIP

## Keyword Mapping

- False
    - Amazing, every word of what you just said is wrong
    - Oh, not good
    - Bad motivator
    - **No, that's not true. That's impossible**
    - We don’t serve there kind here
    - That’s no moon
- True
    - What I told you was true from **a certain point of view**
    - Help me Obi-Wan Kenobi, you’re my only hope
    - Search you feelings lord Vader, you know it to be true
- If
    - Try not. Do or do not. There is no try.
    - **Do or do not.**
- Else
    - These aren’t the droids you’re looking for
- EndIf
    - **You have failed me for the last time**
    - Now his failure is complete
- While
    - I'll try spinning, that's a good trick
    - **Here we go again**
- EndWhile
    - Let the past die
- For
    - For a thousand generations the Jedi were guardians of the republic
- EndFor
    - **You cannot escape your destiny**
    - When 900 years old you reach, look as good, you will not
- +
    - This will make a fine addition to my collection
- -
    - We lost something
    - **Proceed with the countdown**
- *
    - There's too many of them
- ^
    - Unlimited power
- /
    - Not to worry, at least we are flying half a ship
- %
    - Never tell me the odds
- ==
    - You're a Jedi too, nice to meet you
- >
    - There is always a bigger fish
- <
    - Impressive. Most impressive
- Or
    - Never tell me the odds
- And
    - There is another
- Not
    - Always with you what cannot be done
- Noop
    - Move along. Move along
- DeclareMethod
    - What is thy bidding, my master
    - **This is where the fun begins**
- NotVoidMethod
    - It's a trap
- MethodArgument
    - Now that’s a name I have not heard in a long time, a long time
- Return
    - There is nothing for me here now
    - Let the Wookiee win
    - **You’re all clear kid, let's blow this thing and go home**
- EndMethodDeclaration
    - **It is clear to me the Republic no longer functions**
    - You want to go home and rethink your life
    - You know what you need to do
- CallMethod
    - I have a bad feeling about this
    - **I hope you know what you’re doing**
- AssignVariableFromMethodCall
    - You saved me
    - As you wish
    - **Many Bothans died to bring us this information**
- DeclareFloat
    - Yoda, you seek Yoda
- DeclareString
    - Size matters not
- DeclareBool
    - I am the senate
- SetInitialValue
    - The maker!
    - **Whoosa are youssa**
- BeginMain
    - **Do it**
    - This is where the fun begins
- EndMain
    - **May The Force be with you**
- Print
    - Use the Force
    - What is meessa saying
    - **The Sacred Texts!**
- ReadFloat
    - Looking, found someone I would say you have
- ReadString
    - Now this is pod racing
- AssignVariable
    - What a piece of junk
- SetValue
    - I am your father
    - I am a Jedi, like my father before me
- EndAssignVariable
    - The garbage will do
- ParseError
    - Sometimes there are things no one can fix.
    - I trusted them to fix it. It's not my fault
    - Don't blame me, I'm an interpreter
    - I’m not much more than an interpreter
    - You will find it is you that are mistaken about a great many things
    - I felt a great disturbance in the Force
    - I’m standing here in pieces and you’re having dillusions of grandeur
    - Your feeble skills are no match for the power of the dark side
    - You want to go home and rethink your life
    - That's not how the Force works
    - I like where your head's at but no
    - Failure, the greatest teacher is
    - How rude
    - Will this agony ever end?
    - Amazing, every word of what you just said is wrong
    

Things to consider
- It's a trap
- I am the senate
- I’ll try spinning, thats a good trick
- Now this is pod racing
- Size matters not

## Example

With out the keywords

```force
DeclareMethod name
    # void
EndMethodDeclaration

DeclareMethod name
    MethodArgument firstArg
    MethodArgument secondArg
    NonVoidMethod
    
    # Stuff
    Return value
EndMethodDeclaration

BeginMain
    DeclareFloat name
    SetInitialValue value

    AssignVariable name
        SetValue firstOperand
        + secondOperand
        / thirdOperand
    EndAssignVariable

    AssignVariable name
        SetValue firstOperand
        or secondOperand
    EndAssignVariable

    AssignVariableFromMethodCall name
        CallMethod name
    EndAssignVariable

    AssignVariableFromMethodCall name
        CallMethod ReadString
    EndAssignVariable
    
    If value
        # Stuff
    Else
        # Stuff
    EndIf

    While value
        # Stuff
    EndWhile

    For intValueExpre # Auto increments a value?
        # Stuff
    EndFor
    
EndMain
```
