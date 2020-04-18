### FindJar

In linux to find JARs containing the class is done through  
`find /directory/containing/jars -type f -name "*.jar" | xargs grep some.fully.qualified.classname`
  
This repo is doing exactly the same.  
Motivation for this was, it is hard on windows machine to use the above method. However, I haven't tried to build this on windows yet.
