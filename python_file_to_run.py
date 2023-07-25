
class animal():
    def __init__(self):
        print("Animal created")


class dog(animal):

    def __init__(self, name):
        super().__init__()
        self.name = name
       
    def sit(self):
        print(self.name.title() + " is now sitting.")
        
    def roll_over(self):
        print(self.name.title() + " rolled over!")

my_dog = dog('Woofy')
my_dog.sit()
my_dog.roll_over()


print("Is my dog an animal? " + str(isinstance(my_dog, animal)))
print("Is my class a dog? " + str(isinstance(my_dog, dog)))
print("My dog's name is " + my_dog.name.title() + ".")
 