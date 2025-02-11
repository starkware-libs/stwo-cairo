import json

input_data = input().strip()
numbers = input_data.split(',')
hex_list = [hex(int(num)) for num in numbers]
print(json.dumps(hex_list))