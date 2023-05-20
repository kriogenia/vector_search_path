package dev.sotoestevez.index.service

interface QueueService<T> {

    fun enqueue(doc: T): Boolean
    fun enqueue(doc: List<T>): Boolean

}