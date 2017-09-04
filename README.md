# unity_coroutines
An attempt to create something like the Unity game engine's coroutine system, via Rust's new generator syntax.

Unity's coroutine system is useful because it allows you to express complex stateful logic in a single function. The resumable nature of coroutines allow you to suspend the coroutine until some important game event happens, such as splayer input or the passage of time.

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
