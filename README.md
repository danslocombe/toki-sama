# toki sama
A not-very-serious thesaurus for English using toki pona.

## Hosted [here](https://danslocombe.github.io/toki-sama/pages/)

----

[toki pona](https://tokipona.org/) is a language made up of ~130 words.
Its aim is to break down complex ideas into simple elements.
Translating from English to toki pona, many concepts are represented as compound words.

The thesaurus works by translating an English word and looking for other words made of the same "building blocks".
For example, "poetry" is translated as "toki musi" (literally amusing, creative or funny talk.)
The word "joke" can also be translated to to "toki musi." So from toki pona's perspective they are an exact match.

More formally, suppose we have a [vector space we want to embed English words into](https://en.wikipedia.org/wiki/Word_embedding).
Whether the toki pona translation of a word contains a specific toki pona term splits this space into two - either the term is in the translation or it is not.
Each word in toki pona defines its own bisection cutting the space into many smaller regions. By translating a word we can find the region it lays in, and by looking at
English words in the same or "nearby" regions we hope to find words that are "nearby" in semantic meaning.
