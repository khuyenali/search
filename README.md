# Search
1. Use Tf/Idf to rank
2. Boolean query: AND or OR

# Todo
- [ ] Token stemming
- [ ] Support other file types (pdf, txt, ...)
- [ ] Partial term search, wildcard search (?postfix tree)

## Indexing pipeline:
1. Parse: convert html/xml files to string (text only)
2. Tokenization:
   1. Get each token by white space
   2. Remove punctuation
   3. Remove word's length < 3
   4. Remove stop words
   5. (!Todo) Stemming
3. Document count construction: HashMap<Term, int>
4. Insertion: Add document count to the inverted index, also calculate the tf (term frequency)
5. Tf/Idf calculation: Scan through all posting to calculate the Tf/Idf, (also sort the posting lists)

## Search pipeline:
1. Tokenzation (remove punctuation, remove stop words...)
2. Get posting lists of each term, merge the related posting lists (AND or OR)

