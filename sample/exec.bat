javac Sample.java
java -agentlib:jdwp=transport=dt_socket,server=y,suspend=n,address=5005 Sample
