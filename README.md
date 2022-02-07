# toki sama
A not-very-serious thesaurus for English using toki pona.

## Hosted [here](https://danslocombe.github.io/toki-sama/pages/)

----

[toki pona](https://tokipona.org/) is a language made up of ~130 words.
Its aim is to break down complex ideas into simple elements.
Translating from English to toki pona, many concepts are represented as compound words.

The thesaurus works by translating an English word and looking for other words made of the same "building blocks".
For example, "poetry" is translated as the compound "toki musi" (literally amusing, creative or funny talk.)
The word "joke" can also be translated to to "toki musi." So from toki pona's perspective they are an exact match and are similar-ish in english!

---

More formally, we want to find a [vector space we can embed English words into](https://en.wikipedia.org/wiki/Word_embedding).
Each English word should be mapped to a point in the space where if another English word is mapped close they are somehow "close" in semantic meaning.
For example we want "poetry" and "joke" to be at points closer together than "poetry" and "jellyfish".

Whether the toki pona translation of a word contains a specific toki pona word splits this space into two - either the word is in the compound translation or it is not.
For example "musi" splits the English words "poetry" and "joke" into one region and "jellyfish" into the other.
Each word in toki pona defines its own bisection, cutting the space into many smaller regions.
By translating an English word we can find the region it lays in, and by looking at English words in the same or "nearby" regions we hope to find words that are "nearby" in semantic meaning.

---

### Limitations:

We don't take into account word order, or "pi" grouping.
A bunch of translations are (badly) mined from the toki pona corpus with this assumption.

### Fun possibilitites 

Could we do bitwise operations on the toki pona translations?
We could define `poetry | cat` as: translate "poetry" and "jellyfish" into "toki musi" "soweli" then look for English words that map near "toki musi soweli".