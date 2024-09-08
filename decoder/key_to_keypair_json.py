import base58
import json
import sys

def decode_private_key(encoded_key):
    byte_array = base58.b58decode(encoded_key)
    integer_list = [int(byte) for byte in byte_array]
    return integer_list

if __name__ == "__main__":
    if len(sys.argv) != 1:
        print("Usage: python script.py")
        sys.exit(1)
    
    encoded_key = input("Enter your base58-encoded private key: ").strip()

    decoded_key = decode_private_key(encoded_key)

    # Save the array part to a JSON file without indentation and spaces
    output_filename = "decoded_private_key.json"
    with open(output_filename, "w") as output_file:
        json.dump(decoded_key, output_file, separators=(",", ":"))

    print(f"Result successfully saved to file: {output_filename}")
