
using UnityEngine;
using UnityEngine.UI;
//Esteban
public class PlayerType : MonoBehaviour
{

	/*Random
	 *Human
	 *Rollout
	 *Dijkstra
	 *TabularQLearning
	 */

	public static PlayerType MyPlayersType;
	public int typeP1= 0;
	public int typeP2= 1;
	public Text typeP1txt;
	public Text typeP2txt;
	
	void Start ()
	{
		MyPlayersType = this;
	}
	
	// Update is called once per frame
	void Update ()
	{

	}
}
