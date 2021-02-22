package com.example.jetpackcomposehelloworld

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.setContent
import androidx.compose.ui.res.imageResource
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.example.jetpackcomposehelloworld.ui.theme.JetpackComposeHelloWorldTheme

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            JetpackComposeHelloWorldTheme {
                // A surface container using the 'background' color from the theme
                Surface(color = MaterialTheme.colors.background) {
                    NewsStory()
                }
            }
        }
    }
}

@Composable
fun Counter(count: Int, updater: (Int) -> Unit) {
    Button(onClick = { updater(count + 1) }) {
        Text("I have been clicked $count times!")
    }
}

@Composable
fun NewsStory() {
    val counterCount = remember { mutableStateOf(0) }

    val image = imageResource(R.drawable.header)
    MaterialTheme {
        val typography = MaterialTheme.typography

        Column(modifier = Modifier.padding(16.dp)) {
            Image(image, modifier = Modifier
                    .preferredHeight(180.dp)
                    .fillMaxWidth()
                    .clip(shape = RoundedCornerShape(4.dp)), contentScale = ContentScale.Crop)

            Spacer(Modifier.preferredHeight(16.dp))

            Text("A day wandering through the sandhills " +
                    "in Shark Fin Cove, and a few of the " +
                    "sights I saw",
                    style = typography.h6,
                    maxLines = 2,
                    overflow = TextOverflow.Ellipsis)

            Text("London, UK", style = typography.body2)
            Text("Feb 2021", style = typography.body2)
            Divider(thickness = 32.dp)
            Counter(counterCount.value) { newCount -> counterCount.value = newCount }
        }
    }
}

@Preview(showBackground = true)
@Composable
fun DefaultPreview() {
    JetpackComposeHelloWorldTheme {
        NewsStory()
    }
}