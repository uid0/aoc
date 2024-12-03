from collections import Counter

def calculate_similarity_score(left_list, right_list):
    # Count the occurrences of each number in the right list
    right_list_count = Counter(right_list)
    
    # Initialize the total similarity score
    total_similarity_score = 0
    
    # For each number in the left list, calculate its contribution to the similarity score
    for number in left_list:
        # Multiply the number by its count in the right list and add to the total score
        total_similarity_score += number * right_list_count[number]
    
    return total_similarity_score

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

# Calculate the total similarity score
similarity_score = calculate_similarity_score(left_list, right_list)

# Output the result
print(f"The total similarity score is: {similarity_score}")

