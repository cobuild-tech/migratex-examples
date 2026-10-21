# Tomcat 9.0.44 on OpenJDK 8, the server in the upstream IntelliJ run
# configuration (.idea/runConfigurations/Tomcat_9_0_44.xml), after the last
# upstream commit moved the code back to Java 8 for Tomcat 9.
# The tag is amd64 only; Docker runs it emulated on arm64.
FROM --platform=linux/amd64 tomcat:9.0.44-jdk8-openjdk

# pom.xml asks for Connector/J 8.0.22. Hibernate is in the pom too, but nothing uses it.
ADD https://repo1.maven.org/maven2/mysql/mysql-connector-java/8.0.22/mysql-connector-java-8.0.22.jar /tmp/

# The app is deployed as ROOT, because its links are absolute (/index.jsp).
COPY web /usr/local/tomcat/webapps/ROOT
COPY src/main/java /tmp/src
RUN mkdir -p webapps/ROOT/WEB-INF/classes webapps/ROOT/WEB-INF/lib \
 && javac -source 1.8 -target 1.8 -cp lib/servlet-api.jar \
      -d webapps/ROOT/WEB-INF/classes $(find /tmp/src -name '*.java') \
 && cp /tmp/mysql-connector-java-8.0.22.jar webapps/ROOT/WEB-INF/lib/ \
 && rm -rf /tmp/src
