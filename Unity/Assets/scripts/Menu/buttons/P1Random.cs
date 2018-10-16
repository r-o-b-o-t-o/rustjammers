using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
//Esteban
public class P1Random : MonoBehaviour {

	public Button p1Random;

	// Use this for initialization
	void Start () {

		Button btn1 = p1Random.GetComponent<Button>();
		btn1.onClick.AddListener(TaskOnClick);
	}
	
	// Update is called once per frame
	void Update () {
		
	}
	
	void TaskOnClick()
	{
		PlayerType.MyPlayersType.typeP1txt.text = "Random";
		PlayerType.MyPlayersType.typeP1 = 0;
	}
}
