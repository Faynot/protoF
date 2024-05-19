def main():
    name = input("Введите ваше имя: ")
    age = int(input("Введите ваш возраст: "))
    if age >= 18:
    	print("Добро пожаловать, ", name, "!")
    	print("Вы уже совершеннолетний.")
    else:
    	print("Привет, ", name, "!")
    	print("Вы еще несовершеннолетний.")

def add_numbers(num1, num2):
	return num1 + num2

def multiply_numbers(num1, num2):
	return num1 * num2

num1 = int(input("Введите первое число: "))
num2 = int(input("Введите второе число: "))

print("Сумма чисел:", add_numbers(num1, num2))
print("Произведение чисел:", multiply_numbers(num1, num2))

if __name__ == "__main__":
	main()
