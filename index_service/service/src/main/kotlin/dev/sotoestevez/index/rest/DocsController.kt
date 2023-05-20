package dev.sotoestevez.index.rest

import dev.sotoestevez.index.dto.MovieDoc
import dev.sotoestevez.index.dto.MovieDocs
import dev.sotoestevez.index.response.PostDocsResponse
import dev.sotoestevez.index.service.QueueService
import org.springframework.http.HttpStatus
import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping("/docs")
class DocsController(val queueService: QueueService<MovieDoc>) {

    @PostMapping("", "/")
    fun postDocs(@RequestBody body: MovieDocs): ResponseEntity<PostDocsResponse> {
        if (queueService.enqueue(body.docs)) {
            return ResponseEntity(PostDocsResponse(true), HttpStatus.ACCEPTED)
        }
        return ResponseEntity(PostDocsResponse(false), HttpStatus.INTERNAL_SERVER_ERROR)
    }

}