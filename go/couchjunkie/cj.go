package main

import (
	"log"
	"fmt"
	"database/sql"	
)
import _ "github.com/go-sql-driver/mysql"

func main() {
	db, err := sql.Open("mysql", "ignitus:dbpass@tcp(autoarcade:3306)/couchjunkie")
    if( err != nil ) {
    	fmt.Printf("Error\n")
    	log.Fatal(err)
    }  else {
    	fmt.Printf("Success Connecting to Database.\n\n")
 	 	rows,err := db.Query("show tables;")
 	 	if( err != nil ) {
			log.Fatal(err)
		}
		fmt.Printf("Tables:\n")
 	 	for rows.Next() {
			var result string
			if err := rows.Scan(&result); err != nil {
				log.Fatal(err)
			}
			fmt.Printf("%s\n", result)
		}

    }
}
