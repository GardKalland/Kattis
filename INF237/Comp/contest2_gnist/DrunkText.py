def hide_text(drunk_text, innocuous_text):
    drunk_ptr = 0
    innocuous_ptr = 0
    message = ""

    for char in drunk_text:
        if drunk_ptr < len(drunk_text) and char == drunk_text[drunk_ptr]:
            drunk_ptr += 1
        elif innocuous_ptr < len(innocuous_text) and char == innocuous_text[innocuous_ptr]:
            innocuous_ptr += 1
        else:
            message += char

    message += innocuous_text[innocuous_ptr:]

    return message

# Example usage
drunk_text = "xoxo"
innocuous_text = "xerox"
message = hide_text(drunk_text, innocuous_text)
print(message)