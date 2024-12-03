def calculate_total_distance(left_list, right_list):
    # Sort both lists
    left_list.sort()
    right_list.sort()
    
    # Calculate the total distance
    total_distance = 0
    for left, right in zip(left_list, right_list):
        total_distance += abs(left - right)
    
    return total_distance

def read_input_file(file_path):
    # Read the input file and parse it into two lists
    left_list = []
    right_list = []
    
    with open(file_path, 'r') as file:
        # Read each line in the file
        for line in file:
            # Split the line into two integers and append to respective lists
            left, right = map(int, line.split())
            left_list.append(left)
            right_list.append(right)
    
    return left_list, right_list

# Main execution
file_path = 'input.txt'
left_list, right_list = read_input_file(file_path)

# Calculate the total distance
total_distance = calculate_total_distance(left_list, right_list)

# Output the result
print(f"The total distance between the lists is: {total_distance}")

