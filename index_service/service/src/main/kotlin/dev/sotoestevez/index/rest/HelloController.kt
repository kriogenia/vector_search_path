package dev.sotoestevez.index.rest

import dev.sotoestevez.index.HelloService
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.RestController

@RestController
class HelloController(val service: HelloService) {

    @GetMapping("/")
    fun ping(): String {
        return service.ping()
    }

}