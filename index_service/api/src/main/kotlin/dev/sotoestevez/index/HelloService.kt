package dev.sotoestevez.index

import org.springframework.stereotype.Service

@Service
class HelloService {
    fun ping(): String {
        return "pong"
    }

}