package main

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/base64"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/swaggo/files"
	"github.com/swaggo/gin-swagger"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)



// Token model
type Token struct {
	ID        string    `gorm:"primaryKey" json:"id"`
	Date      time.Time `json:"date"`
	Time      string    `json:"time"`
	Batch     string    `json:"batch"`
	CreatedAt time.Time `json:"created_at"`
}

var (
	db  *gorm.DB
	key []byte
)

// @title API Encode/Decode Example
// @version 1.0
// @description API em Go para encriptar/decriptar dados com Postgres
// @host localhost:8080
// @BasePath /
func main() {
	// --- ENV ---
	dsn := os.Getenv("DATABASE_URL")
	if dsn == "" {
		dsn = "postgres://postgres:postgres@localhost:5432/api_go?sslmode=disable"
	}
	k := os.Getenv("ENCRYPTION_KEY")
	if k == "" {
		panic("ENCRYPTION_KEY não definido (32 bytes base64)")
	}
	var err error
	key, err = base64.StdEncoding.DecodeString(k)
	if err != nil || len(key) != 32 {
		panic("ENCRYPTION_KEY inválido (precisa ter 32 bytes base64)")
	}

	// --- DB ---
	db, err = gorm.Open(postgres.Open(dsn), &gorm.Config{})
	if err != nil {
		panic("Erro conectando ao banco: " + err.Error())
	}
	db.AutoMigrate(&Token{})

	// --- ROUTER ---
	r := gin.Default()

	// Swagger
	r.GET("/swagger/*any", ginSwagger.WrapHandler(swaggerFiles.Handler))

	// Endpoints
	r.POST("/encode", encodeHandler)
	r.GET("/decode/:id", decodeHandler)

	log.Println("Servidor rodando em http://localhost:8080 ...")
	r.Run(":8080")
}

// Request payload
type EncodeRequest struct {
	Date  string `json:"date" binding:"required"`
	Time  string `json:"time" binding:"required"`
	Batch string `json:"batch" binding:"required"`
}

// Response
type EncodeResponse struct {
	ID string `json:"id"`
}

// @Summary Encode data
// @Accept json
// @Produce json
// @Param data body EncodeRequest true "Dados"
// @Success 200 {object} EncodeResponse
// @Router /encode [post]
func encodeHandler(c *gin.Context) {
	var req EncodeRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	parsedDate, _ := time.Parse("2006-01-02", req.Date)
	now := time.Now()

	// Criar payload simples
	payload := fmt.Sprintf("%s|%s|%s|%d", req.Date, req.Time, req.Batch, now.UnixNano())
	id, err := encrypt(payload)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "erro ao encriptar"})
		return
	}

	token := Token{ID: id, Date: parsedDate, Time: req.Time, Batch: req.Batch, CreatedAt: now}
	db.Create(&token)

	c.JSON(http.StatusOK, EncodeResponse{ID: id})
}

// @Summary Decode data
// @Produce json
// @Param id path string true "Token ID"
// @Success 200 {object} Token
// @Router /decode/{id} [get]
func decodeHandler(c *gin.Context) {
	id := c.Param("id")
	var token Token
	if err := db.First(&token, "id = ?", id).Error; err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "não encontrado"})
		return
	}
	c.JSON(http.StatusOK, token)
}

func encrypt(plaintext string) (string, error) {
	block, err := aes.NewCipher(key)
	if err != nil {
		return "", err
	}
	gcm, err := cipher.NewGCM(block)
	if err != nil {
		return "", err
	}
	nonce := make([]byte, gcm.NonceSize())
	if _, err := io.ReadFull(rand.Reader, nonce); err != nil {
		return "", err
	}
	ciphertext := gcm.Seal(nonce, nonce, []byte(plaintext), nil)
	return base64.RawURLEncoding.EncodeToString(ciphertext), nil
}
