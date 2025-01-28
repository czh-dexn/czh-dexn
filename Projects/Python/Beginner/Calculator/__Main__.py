# This is the main file that will run the calculator
from colorama import init, Fore, Back, Style
from InquirerPy import prompt
import sys

# importing our functions
from Functions import Standard
from Functions.Graphical import GraphicalCalculator

# Standard options that the user can choose from
def Standard_Option():
    # declaring the choices we have
    # This is a library that allows us to use the terminal to ask the user for input
    choices = ["Addition", "Subtraction", "Multiplication", "Division", "return to main menu"]
    
    # Starting with the intial question
    question = [
        {
            "type": "list",
            "message": "What operation would you like to perform?",
            "name": "operation",
            "choices": choices
        }
    ]
    
    # Ask the user for the operation, and store the answer
    answer = prompt(question)
    selected_operation = answer["operation"]
    print(f"Selected operation: {selected_operation}")
    
    if selected_operation == "return to main menu":
        return Options()
    else:
        # Ask the user for the numbers
        num1 = float(input("Enter the first number: ").strip('\r'))
        num2 = float(input("Enter the second number: ").strip('\r'))
        
        # Create an instance of the Operations class
        operations_instance = Standard.Operations()
        
        if selected_operation in operations_instance.operations:
            operation_method = operations_instance.operations.get(selected_operation)
            
            if operation_method:
                result = operation_method(num1, num2)
                print(Fore.RED + f"The result of the operation is: {result}")
        else:
            print("Invalid operation")
        
def Options():
    # Giving the user the option to choose betwee5n the standard calculator, graphical calculator, or exit
    Options = ["Standard", "Graphical", "Scientific", "Exit"]
    
    Question = [
        {
            "type": "list",
            "message": "What calculator would you like to use?",
            "name": "calculator",
            "choices": Options
        }
    ]
    
    # Ask the user for the calculator, and store the answer
    Answer = prompt(Question)
    Selected_Calculator = Answer["calculator"]
    print(f"Selected calculator: {Selected_Calculator}")
    
    if Selected_Calculator == "Exit":
        print("Exiting the calculator")
        sys.exit(0)
    elif Selected_Calculator == "Standard":
        Standard_Option()
    elif Selected_Calculator == "Graphical":
        GraphicalCalculator.Startup()
    elif Selected_Calculator == "Scientific":
        print("Scientific calculator not yet implemented")
    
if __name__ == "__main__":
    print("Welcome to the calculator!")
    init()
    
    while True:
        Options()
