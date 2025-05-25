# The decimal numbers
decimal_num1 = 263561599766550617289250058199814760685
decimal_num2 = 65303172752238645975888084098459749904

# Convert to hex and remove the '0x' prefix
hex_result = hex(decimal_num1)[2:]
hex_result_2 = hex(decimal_num2)[2:]

# Concatenate the results
concatenated = hex_result + hex_result_2

print(f"Hexadecimal 1: {hex_result}")
print(f"Hexadecimal 2: {hex_result_2}")
print(f"Concatenated: {concatenated}")