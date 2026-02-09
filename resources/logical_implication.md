---
title: Logical implication
date: 2026-02-09
author: Hong Quan
tags: [math]
summary: A false proposition implies any proposition
---

First off, by a "proposition" I simply mean an expression that has a clear
Boolean value. Propositions can be combined into larger propositions by using
the "not, "and" and "or" operations that we are all familiar with. "Socrates
is a cat or it is raining" is true if either of its sub-propositions are true,
and false otherwise. 

Those are operations are all uncontroversial. But we run into big problems
when we try to come up with a definition for a proposition of the form "if P
then Q", or, as it is traditionally notated, P -> Q. P here is the
"antecedent" and Q is the "consequent".

**The first thing we have to emphasized is that the implication operation does
not suggest that there is a causal relationship**. We usually speak of
implications that have a causal relationship: "if it is raining then the
streets are wet". But the implication proposition "if it is raining then
Minerva is a cat" needs to have a truth value irrespective of the fact that
there is no obvious causal connection between the weather in Seattle and the
name of my elderly cat. 

## Explanation
So what then should be the rules for the truth value of P -> Q, given truth
values for P and Q?

Plainly if P is true and Q is false then the implication was false. If we have
a situation where the antecedent is true but the consequent is false then the
implication was a false statement. That is, if some day it is raining but the
streets are, bizarrely, not wet, the "if it is raining the streets are wet"
must have been false. 

Similarly if P is true and Q is true then the implication was true. If it is
the case that it is raining and Minerva is a cat then we can conclude that the
implication proposition is true, end of story. Again, remember, the truth of
the implication is simply that if the antecedent was true then the consequent
turned out to be true, not that there was a causal connection.

That's pretty uncontroversial. The problems arise when the antecedent is
false. What is the truth value of P -> Q when P is false? In traditional
propositional logic an implication proposition with a false antecedent is
considered a true statement, regardless of the truth of the consequent. That
is, "if Socrates is immortal then the streets are wet" is a true proposition,
regardless of whether the streets are wet or not. 

Many people find this unsettling. I did myself for some time. But now it seems
very natural to me. Rather than ask about the truth of a proposition, let me
tweak it slightly. Suppose I tell you "if it is raining then I wear rubber
boots". *Under what circumstances could you reasonably call me a liar?* If it
is raining and I am not wearing rubber boots then clearly I am a liar. If it
is raining and I am wearing rubber boots then clearly I am not a liar. If it
is not raining then you have nothing upon which to deduce that I am a liar; my
statement is true.

So what does this have to do with programming languages?

I now think of material implication like this:
```c
if (p)
{
    debug.assert(q);
    ...
}
```
We will say that the program is correct (true) if the assertion never fires
and incorrect (false) if the assertion fires. Under what circumstances would
we say that the program is incorrect? Only if p is true and q is false. If p
is false then the assertion never fires, and if both are true then the
assertion never fires. So again, it makes sense to say that P -> Q is true
when P is false.


