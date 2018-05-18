# What is Unity Test Runner

Unity Test Runner is a tool for writing and running tests in Unity (hopefully with command line, integrated with CI). 

Combining this with CI (Continous integration) can help testing code before merging/deploying. CI is big in Open Source projects, because you can check Pull Requests before even reviewing them. Tests are safety net, assurance that new functionality did not break anything. Unfortunately, not all projects use CI. The results are, sometimes someone pushes compile error to main branch.

I have tested UTR (Unity Test Runner) and found it quite useful. You can currently use it for simple tests and introduce it to your workflow, but the tool is still in early stages. I will try to explain more about the details and write about pain points I encountered.

# Not enough documentation/resources

This is the first problem with this tool.
Test Runner is still in early stages (included in Unity since 5.5 version, I believe) and documentation and examples are sparse. There are no good articles about what kind of tests writing for a game. Lot of people don't even know this tool exist. I hope it will change in the future.

# Very slow command line

For some reason, running tests from command line was very slow, especially in Playmode. Playmode tests were all failing because of timeouts, but the timeouts were set to quite high values so I guess there is a bug. It will probably be fixed in future versions.

# Support / Quality

UTR is an official Unity functionality and is included in Unity3D by defaut. It is based on NUnit library which is relatively stable and good quality library. The Unity team is behind the UTR so we can assume basic level of support and quality. It is still in early stages, so we can expect lot of changes, but if the tool is considered to be useful and important, it will continue to be improved.

# Usefulness of UI tests

I am working on UI a lot, and was curious about testing UI. I thought it would be great to be able to test visual aspect of the game.

I was able to write some tests, but then, I was reconsidering their usefulness.

This was the overall structure of simple tests I wrote:

```

[UnityTest]
[Timeout (Time.TenMinutes)]
public IEnumerator BasicTest () {
    yield return GoToScene1 ();
    yield return CheckScene1 ();
}

[Timeout (Time.OneMinute)]
private IEnumerator GoToScene1 () {
    Actions.ClickButton ("Scene1Button");
    yield return Wait.ForSceneOpened<Scene1> ();
}

[Timeout (Time.FiveMinutes)]
private IEnumerator CheckScene1 () {
    yield return Open.AndClosePopup<Popup1>("Popup1Button");
    yield return Open.AndClosePopup<Popup2>("Popup2Button");
    yield return Open.AndClosePopup<Popup3>("Popup3Button");
    yield return Open.AndClosePopup<Popup4>("Popup4Button");
}
```

When I ran those tests, I could catch null reference exceptions (missing references in inspector) or asserts that failed. It was quite useful, but writing those tests and framework (UI testing framework) took me too much of the time. But there are more simpler ways to write UI tests checks.

# What tests can I write?

- tests for missing references in inspector
- test for "Missing Script" problem
- tests of Model/Controller (if you use MVC), to check the UI logic
- checking missing prefabs (try load prefab and check for null)
- tests prefabs/asset bundle paths
- various calculations (calculating player stats)
- null checks (crawling player data and assuring that nothing is null)
- server endpoints (does it connect to server? do we get expected response when we send some request?)

# Final notes

I liked the tool, but I would love to see some real life examples and long articles about it. It has great potential, but it is still new. Still, I would encourage you to try it and integrate it into your workflow.

# More info

Official:
https://docs.unity3d.com/Manual/testing-editortestsrunner.html

Youtube:
https://www.youtube.com/watch?v=VqmcWnjreqg
https://www.youtube.com/watch?v=GIJptHunxow

Testing UI:
https://assetstore.unity.com/packages/tools/unity-ui-test-automation-72693

NUnit:
http://nunit.org/

Bugs:
https://forum.unity.com/threads/why-the-regressions-to-test-runner-in-unity-5-6.471548/
https://issuetracker.unity3d.com/issues/running-editor-tests-with-batch-mode-doesnt-display-test-results