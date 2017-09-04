# unity_coroutines
An attempt to create something like the Unity game engine's coroutine system, via Rust's new generator syntax.

Unity's coroutine system is useful because it allows you to express complex stateful logic in a single function. The resumable nature of coroutines allows the programmer to suspend the coroutine until some important game event happens, such as player input or the passage of time.

The most obvious alternative is to mantually write a state machine. This is less than ideal for a number of reasons. Just imagine trying to write a state machine with equivalent functionality to the following unity coroutine, which runs a simple tutorial:

```c#
// Example of a unity/C# coroutine
private IEnumerator PlayTutorial() {
    ShowMessage("Press the space bar to attack");

    bool complete = false;
    int failures = 0;

    while(!complete) {
        // wait until the player presses a key
        yield return new WaitUntil(() => Input.anyKeyDown);

        if(Input.GetKeyDown(KeyCode.Esc)) {
            // the player has pressed esc, quit the tutorial
            yield break;
        }
        else if(Input.GetKeyDown(KeyCode.Space)) {
            // the player has pressed space, move on to the next phase
            complete = true;
        }
        else {
            // the player pressed a key, but it was the wrong key, re-iterate the instructions
            ShowMessage("Use the space bar to attack the enemy to your right");
            failures++;
        }
    }

    if(failures > 5) {
        // the player got it, but it took them several tries. give them more practice
        ShowMessage("You did it! Let's try a few more times for practice.");
        yield return new WaitForSeconds(5);

        // Do another loop of the tutorial
        complete = false;
        while(!complete) {
            // wait until the player presses a key
            yield return new WaitUntil(() => Input.anyKeyDown);

            if(Input.GetKeyDown(KeyCode.Esc)) {
                // the player has pressed esc, quit the tutorial
                yield break;
            }
            else if(Input.GetKeyDown(KeyCode.Space)) {
                // the player has pressed space, move on to the next phase
                complete = true;
            }
            else {
                // the player pressed a key, but it was the wrong key, re-iterate the instructions
                ShowMessage("Use the space bar to attack the enemy to your right");
            }
        }
    }

    ShowMessage("Great job! Let's move on to the next section");
}
```
Trying to implement the above as a state machine would be a complete nightmare, both when writing it and when reading it a year later. Unity's coroutines do a great job at making it easier to write code without sacrificing any power/expressivity, nor sacrificing readbility/maintainability. In my opinion, coroutines are one of Unity's most powerful features.

If there was a game engine in Rust that had this capability out of the box, it would go a long way towards making Rust a viable language for game progrmaming. My hope is that this quick project will inspire other Rust developers to take this further.
