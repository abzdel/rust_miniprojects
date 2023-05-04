## Instructions on how to use


1. enter generate_text directory from main folder
```
cd generate_text/
```

2. run cargo run (use --release flag for smaller binary)
```
cargo run --release
```

3. enter text you want the model to generate from:
```
Enter some text for the model to generate from:
The dog (my example)
```

Results will look something like this:
```
[GeneratedTextOutput { text: "The dog\n\nThe dog is a dog that has been trained to be aggressive.\n\nIt is a puppy that has a strong sense of smell, a sense of touch, and a sense that people are trying to touch it. It is also a dog who has a sense", score: None }]
```
