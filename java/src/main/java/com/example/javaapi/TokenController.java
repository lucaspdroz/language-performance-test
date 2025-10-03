package com.example.javaapi;

import org.springframework.web.bind.annotation.*;
import org.springframework.beans.factory.annotation.Autowired;

import java.time.LocalDate;
import java.time.LocalTime;
import java.util.UUID;

@RestController
public class TokenController {

    @Autowired
    private TokenRepository repository;

    @PostMapping("/encode")
    public Token encode(@RequestBody TokenRequest request) {
        Token token = new Token();
        token.setId(UUID.randomUUID().toString());
        token.setDate(request.getDate());
        token.setTime(request.getTime());
        token.setBatch(request.getBatch());

        return repository.save(token);
    }

    @GetMapping("/decode/{id}")
    public Token decode(@PathVariable String id) {
        return repository.findById(id).orElseThrow(() -> new RuntimeException("Token not found"));
    }
}
