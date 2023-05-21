FROM gradle:8-jdk17 AS build

COPY . /tmp
WORKDIR /tmp

RUN ./gradlew build --no-daemon

FROM eclipse-temurin:17-jdk-alpine

EXPOSE 8080

RUN mkdir /app
COPY --from=build /tmp/service/build/libs/*.jar /app/app.jar

ENTRYPOINT ["java", "-jar", "/app/app.jar"]