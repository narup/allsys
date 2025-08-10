from tokenizer_v1 import SimpleTokenizerV1

import re
import urllib.request

def build_vocab():
    url = ("https://raw.githubusercontent.com/rasbt/" "LLMs-from-scratch/main/ch02/01_main-chapter-code/" "the-verdict.txt")
    file_path = "the-verdict.txt"
    urllib.request.urlretrieve(url, file_path)

    with open("the-verdict.txt", "r", encoding="utf-8") as f:
        raw_text = f.read()
        print("Total number of character:", len(raw_text))

    preprocessed = re.split(r'([,.:;?_!"()\']|--|\s)', raw_text)
    preprocessed = [item.strip() for item in preprocessed if item.strip()]
    print(len(preprocessed))

    all_words = sorted(set(preprocessed))
    vocab_size = len(all_words)
    print(vocab_size)

    vocab = {token:integer for integer,token in enumerate(all_words)}
    for i, item in enumerate(vocab.items()):
        print(item)
        if i >= 50:
            break

    return vocab


def main():
    print("Hello from bllm!")
    vocab = build_vocab()
    tokenizer = SimpleTokenizerV1(vocab)
    text = """"It's the last he painted, you know"
               Mrs. Gisburn said with pardonable pride."""
    ids = tokenizer.encode(text)
    print(ids)

if __name__ == "__main__":
    main()

