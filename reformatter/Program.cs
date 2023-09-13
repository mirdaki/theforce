using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Net.Http;
using System.Threading;
namespace reformatter
{
    class Program
    {
        private static List<string> forceCommands = new()
    {
        "Do it!",
        "May The Force be with you.",
        "The Sacred Jedi Texts!",
        "This is where the fun begins.",
        "Now, that's a name I've noe heard in a long time. A long time.",
        "It's a trap!",
        "You're all clear, kid. Now let's blow this thing and go home.",
        "You cannot escape your destiny.",
        "What a piece of junk!",
        "Many Bothans died to bring us this information.",
        "The garbage will do.",
        "I am your father.",
        "I have a bad feeling about this.",
        "Always with you it cannot be done.",
        "Your lightsabers will make a fine addition to my collection.",
        "Proceed with the countdown.",
        "There's too many of them!",
        "Not to worry, we are still flying half a ship.",
        "Unlimited power!",
        "Never tell me the odds!",
        "I am a Jedi, like my father before me.",
        "Impressive. Most impressive.",
        "There's always a bigger fish.",
        "There is another.",
        "As you wish.",
        "Size matters not.",
        "Yoda. You seek Yoda.",
        "I am the Senate!",
        "Who, mesa?",
        "Move along. Move along.",
        "Here we go again.",
        "Let the past die.",
        "Do, or do not. There is no try.",
        "These aren't the droids you're looking for.",
        "You have failed me for the last time.",
        "I'll try spinning, that's a good trick.",
        "For over a thousand generations.",
        "Let the Wookiee win.",
        "It is clear to me now the Republic no longer functions.",
        "Now this is podracing!",
        "Looking? Found someone, you have, I would say.",
        "I hope you know what you're doing."
    };

        private static List<string> pseudoCommands = new()
    {
        "BeginMain",
        "EndMain",
        "Print",
        "DeclareFunction",
        "FunctionParameters",
        "Void",
        "Return",
        "EndFunctionDeclaration",
        "AssignVariable",
        "AssignVariableFromFunctionCall",
        "EndAssignVariable",
        "SetValue",
        "CallFunction",
        "Not",
        "Add",
        "Subtract",
        "Multiply",
        "Divide",
        "Exponent",
        "Modulus",
        "Equal",
        "GreaterThan",
        "LessThan",
        "Or",
        "And",
        "DeclareFloat",
        "DeclareString",
        "DeclareBool",
        "SetInitialValue",
        "Noop",
        "While",
        "EndWhile",
        "If",
        "Else",
        "EndIf",
        "PassArgument",
        "For",
        "ForStart",
        "EndFor",
        "ReadFloat",
        "ReadString",
        "ReadBoolean"
    };

        static void Main(string[] args)
        {
            if (args.Length != 4)
            {
                Console.WriteLine("Usage is:");
                Console.WriteLine("<exe> <fileToConvert> <inputFormat> <outputFormat> <destinationLocation>");
                Console.WriteLine("Valid input formats are { force, pseudo }. Valid output formats are { force, pseudo, video }.");
            }

            List<string> inputList = args[1].Equals("force") ? forceCommands :
                                           args[1].Equals("pseudo") ? pseudoCommands :
                                           throw new ArgumentException("Incorrect input type");

            List<string> outputList = args[2].Equals("force") ? forceCommands :
                                      args[2].Equals("pseudo") ? pseudoCommands :
                                      args[2].Equals("video") ? new List<string>() :
                                      throw new ArgumentException("Incorrect output type");

            string[] inputText = File.ReadAllLines(args[0]);
            for (int a = 0; a < inputText.Length; a++)
            {
                for (int b = inputList.Count - 1; b >= 0; b--)
                {
                    int index = inputText[a].IndexOf(inputList[b]);
                    if (index != -1)
                    {
                        if (args[2].Equals("video"))
                        {

                        }
                        else
                        {
                            inputText[a] = inputText[a].Substring(0, index) +
                                outputList[b] +
                                inputText[a].Substring(index + inputList[b].Length, inputText[a].Length - index - inputList[b].Length);
                        }

                        break;
                    }
                }
            }

            File.WriteAllLines(args[3], inputText);
        }
    }

}
