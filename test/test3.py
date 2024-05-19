def calculate_average():
total = 0
count = 0
while True:
number = input("Enter a number (or 'done' to exit): ")
if number == 'done':
break
total += float(number)
count += 1
average = total / count
print("Average:", average)

def main():
print("Welcome to the average calculator!")
calculate_average()

main()
