from Extra.Decorators import log_operation

class Operations():
    def __init__(self) -> None:
        self.operations = {
            "Addition": self.Addition,
            "Subtraction": self.Subtraction,
            "Multiplication": self.Multiplication,
            "Division": self.Division
        }
    
    @log_operation
    def Addition(self, num1, num2):
        return num1 + num2

    @log_operation
    def Subtraction(self, num1, num2):
        return num1 - num2
    
    @log_operation
    def Multiplication(self, num1, num2):
        return num1 * num2
    
    @log_operation
    def Division(self, num1, num2):
        if num2 == 0:
            return "Cannot divide by zero"
        else:
            return num1 / num2
        
    def get_operation(operation_name):
        return Operations(operation_name)